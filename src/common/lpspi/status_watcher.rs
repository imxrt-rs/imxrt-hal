use core::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use cortex_m::interrupt::{self, Mutex};

use super::{ral, LpspiError};

struct StatusWatcherInner<const N: u8> {
    transfer_complete_happened: bool,
    transfer_complete_waker: Option<Waker>,
    error_caught: Option<LpspiError>,
    error_caught_waker: Option<Waker>,
    tx_fifo_watermark_busy: bool,
    rx_fifo_watermark_busy: bool,
    tx_fifo_watermark_waker: Option<Waker>,
    rx_fifo_watermark_waker: Option<Waker>,
    interrupts_enabled: bool,
}

pub(crate) struct StatusWatcher<const N: u8> {
    inner: Mutex<RefCell<StatusWatcherInner<N>>>,
    lpspi: ral::lpspi::Instance<N>,
}

impl<const N: u8> StatusWatcherInner<N> {
    pub fn check_and_reset(&mut self, lpspi: &ral::lpspi::Instance<N>) {
        if ral::read_reg!(ral::lpspi, lpspi, SR, TCF == TCF_1) {
            ral::write_reg!(ral::lpspi, lpspi, SR, TCF: TCF_1);

            self.transfer_complete_happened = true;
            if let Some(waker) = self.transfer_complete_waker.take() {
                waker.wake();
            }
        }

        if ral::read_reg!(ral::lpspi, lpspi, SR, TEF == TEF_1) {
            ral::write_reg!(ral::lpspi, lpspi, SR, TEF: TEF_1);

            self.error_caught = Some(LpspiError::TransmitFifo);
            if let Some(waker) = self.error_caught_waker.take() {
                waker.wake();
            }
        }

        if ral::read_reg!(ral::lpspi, lpspi, SR, REF == REF_1) {
            ral::write_reg!(ral::lpspi, lpspi, SR, REF: REF_1);

            self.error_caught = Some(LpspiError::ReceiveFifo);
            if let Some(waker) = self.error_caught_waker.take() {
                waker.wake();
            }
        }

        if ral::read_reg!(ral::lpspi, lpspi, SR, TDF == TDF_1) {
            ral::modify_reg!(ral::lpspi, lpspi, IER, TDIE: TDIE_0);

            if let Some(waker) = self.tx_fifo_watermark_waker.take() {
                waker.wake();
            }
        }
        if ral::read_reg!(ral::lpspi, lpspi, SR, RDF == RDF_1) {
            ral::modify_reg!(ral::lpspi, lpspi, IER, RDIE: RDIE_0);

            if let Some(waker) = self.rx_fifo_watermark_waker.take() {
                waker.wake();
            }
        }
    }
}

impl<const N: u8> StatusWatcher<N> {
    pub fn new(lpspi: ral::lpspi::Instance<N>) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(StatusWatcherInner {
                transfer_complete_happened: false,
                transfer_complete_waker: None,
                error_caught: None,
                error_caught_waker: None,
                tx_fifo_watermark_busy: false,
                rx_fifo_watermark_busy: false,
                tx_fifo_watermark_waker: None,
                rx_fifo_watermark_waker: None,
                interrupts_enabled: false,
            })),
            lpspi,
        }
    }

    pub fn enable_interrupts(&self) {
        interrupt::free(|cs| {
            let inner = self.inner.borrow(cs);
            let mut inner = inner.borrow_mut();

            ral::write_reg!(ral::lpspi, self.lpspi, IER,
                REIE: REIE_1,
                TEIE: TEIE_1,
                TCIE: TCIE_1
            );

            inner.interrupts_enabled = true;
        });
    }

    #[inline]
    pub fn instance(&self) -> &ral::lpspi::Instance<N> {
        &self.lpspi
    }

    fn with_check_and_reset<R>(&self, f: impl FnOnce(&mut StatusWatcherInner<N>) -> R) -> R {
        interrupt::free(|cs| {
            let inner = self.inner.borrow(cs);
            let mut inner = inner.borrow_mut();
            inner.check_and_reset(&self.lpspi);

            f(&mut inner)
        })
    }

    pub fn on_interrupt(&self) {
        self.with_check_and_reset(|_| {});
    }

    pub fn clear_transfer_complete(&self) {
        self.with_check_and_reset(|inner| {
            inner.transfer_complete_happened = false;
        });
    }

    pub async fn wait_transfer_complete(&self) {
        StatusWatcherFuture::new(
            self,
            |inner| inner.transfer_complete_happened.then_some(()),
            |inner| &mut inner.transfer_complete_waker,
            |_| (),
            |_| (),
        )
        .await
    }

    pub async fn watch_for_errors(&self) -> Result<(), LpspiError> {
        let error = StatusWatcherFuture::new(
            self,
            |inner| inner.error_caught.take(),
            |inner| &mut inner.error_caught_waker,
            |_| (),
            |_| (),
        )
        .await;

        Err(error)
    }

    pub fn check_for_errors(&self) -> Result<(), LpspiError> {
        match self.with_check_and_reset(|inner| inner.error_caught.take()) {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    pub fn clear_errors(&self) {
        self.with_check_and_reset(|inner| {
            inner.error_caught = None;
        });
    }

    pub async fn wait_for_tx_watermark(&self) -> Result<(), LpspiError> {
        self.with_check_and_reset(|inner| -> Result<(), LpspiError> {
            if inner.tx_fifo_watermark_busy {
                Err(LpspiError::Busy)
            } else {
                inner.tx_fifo_watermark_busy = true;
                Ok(())
            }
        })?;

        Ok(StatusWatcherFuture::new(
            self,
            |_| ral::read_reg!(ral::lpspi, self.lpspi, SR, TDF == TDF_1).then_some(()),
            |inner| &mut inner.tx_fifo_watermark_waker,
            |_| ral::modify_reg!(ral::lpspi, self.lpspi, IER, TDIE: TDIE_1),
            |inner| {
                ral::modify_reg!(ral::lpspi, self.lpspi, IER, TDIE: TDIE_0);
                inner.tx_fifo_watermark_busy = false;
            },
        )
        .await)
    }

    pub async fn wait_for_rx_watermark(&self, watermark: u32) -> Result<(), LpspiError> {
        self.with_check_and_reset(|inner| -> Result<(), LpspiError> {
            if inner.rx_fifo_watermark_busy {
                Err(LpspiError::Busy)
            } else {
                inner.rx_fifo_watermark_busy = true;
                Ok(())
            }
        })?;

        interrupt::free(|_| {
            ral::modify_reg!(ral::lpspi, self.lpspi, FCR, RXWATER: watermark);
        });

        Ok(StatusWatcherFuture::new(
            self,
            |_| ral::read_reg!(ral::lpspi, self.lpspi, SR, RDF == RDF_1).then_some(()),
            |inner| &mut inner.rx_fifo_watermark_waker,
            |_| ral::modify_reg!(ral::lpspi, self.lpspi, IER, RDIE: RDIE_1),
            |inner| {
                ral::modify_reg!(ral::lpspi, self.lpspi, IER, RDIE: RDIE_0);
                inner.rx_fifo_watermark_busy = false;
            },
        )
        .await)
    }
}

struct StatusWatcherFuture<'a, const N: u8, T, C, W, I, D>
where
    C: Fn(&mut StatusWatcherInner<N>) -> Option<T>,
    W: Fn(&mut StatusWatcherInner<N>) -> &mut Option<Waker>,
    I: Fn(&mut StatusWatcherInner<N>),
    D: Fn(&mut StatusWatcherInner<N>),
{
    watcher: &'a StatusWatcher<N>,
    condition: C,
    waker: W,
    interrupt_enable: I,
    on_drop: D,
}

impl<'a, const N: u8, T, C, W, I, D> StatusWatcherFuture<'a, N, T, C, W, I, D>
where
    C: Fn(&mut StatusWatcherInner<N>) -> Option<T>,
    W: Fn(&mut StatusWatcherInner<N>) -> &mut Option<Waker>,
    I: Fn(&mut StatusWatcherInner<N>),
    D: Fn(&mut StatusWatcherInner<N>),
{
    fn new(
        watcher: &'a StatusWatcher<N>,
        condition: C,
        waker: W,
        interrupt_enable: I,
        on_drop: D,
    ) -> Self {
        Self {
            watcher,
            condition,
            waker,
            interrupt_enable,
            on_drop,
        }
    }
}

impl<'a, const N: u8, T, C, W, I, D> Future for StatusWatcherFuture<'a, N, T, C, W, I, D>
where
    C: Fn(&mut StatusWatcherInner<N>) -> Option<T>,
    W: Fn(&mut StatusWatcherInner<N>) -> &mut Option<Waker>,
    I: Fn(&mut StatusWatcherInner<N>),
    D: Fn(&mut StatusWatcherInner<N>),
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.watcher.with_check_and_reset(|inner| {
            if let Some(result) = (self.condition)(inner) {
                Poll::Ready(result)
            } else {
                let new_waker = cx.waker();

                // From embassy
                // https://github.com/embassy-rs/embassy/blob/b99533607ceed225dd12ae73aaa9a0d969a7365e/embassy-sync/src/waitqueue/waker.rs#L59-L61
                match (self.waker)(inner) {
                    // Optimization: If both the old and new Wakers wake the same task, we can simply
                    // keep the old waker, skipping the clone. (In most executor implementations,
                    // cloning a waker is somewhat expensive, comparable to cloning an Arc).
                    Some(w2) if (w2.will_wake(new_waker)) => {}
                    _ => {
                        // clone the new waker and store it
                        if let Some(old_waker) = (self.waker)(inner).replace(new_waker.clone()) {
                            // We had a waker registered for another task. Wake it, so the other task can
                            // reregister itself if it's still interested.
                            //
                            // If two tasks are waiting on the same thing concurrently, this will cause them
                            // to wake each other in a loop fighting over this WakerRegistration. This wastes
                            // CPU but things will still work.
                            //
                            // If the user wants to have two tasks waiting on the same thing they should use
                            // a more appropriate primitive that can store multiple wakers.
                            old_waker.wake()
                        }
                    }
                }

                // If interrupts are disabled, notify right away to provoke busy-waiting
                if !inner.interrupts_enabled {
                    if let Some(waker) = (self.waker)(inner).take() {
                        waker.wake();
                    }
                } else {
                    (self.interrupt_enable)(inner);
                }

                Poll::Pending
            }
        })
    }
}

impl<'a, const N: u8, T, C, W, I, D> Drop for StatusWatcherFuture<'a, N, T, C, W, I, D>
where
    C: Fn(&mut StatusWatcherInner<N>) -> Option<T>,
    W: Fn(&mut StatusWatcherInner<N>) -> &mut Option<Waker>,
    I: Fn(&mut StatusWatcherInner<N>),
    D: Fn(&mut StatusWatcherInner<N>),
{
    fn drop(&mut self) {
        self.watcher
            .with_check_and_reset(|inner| (self.on_drop)(inner));
    }
}

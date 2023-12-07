use core::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use cortex_m::interrupt::{self, Mutex};

use super::ral;

struct StatusWatcherInner<const N: u8> {
    transfer_complete_happened: bool,
    transfer_complete_waker: Option<Waker>,
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
    }
}

impl<const N: u8> StatusWatcher<N> {
    pub fn new(lpspi: ral::lpspi::Instance<N>) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(StatusWatcherInner {
                transfer_complete_happened: false,
                transfer_complete_waker: None,
                interrupts_enabled: false,
            })),
            lpspi,
        }
    }

    pub fn enable_interrupts(&self) {
        interrupt::free(|cs| {
            let inner = self.inner.borrow(cs);
            let mut inner = inner.borrow_mut();
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

    // pub fn poll_transfer_complete(&self) -> bool {
    //     self.with_check_and_reset(|inner| inner.transfer_complete_happened)
    // }

    pub fn wait_transfer_complete(&self) -> WaitTransferComplete<N> {
        WaitTransferComplete(self)
    }
}

pub struct WaitTransferComplete<'a, const N: u8>(&'a StatusWatcher<N>);

impl<const N: u8> Future for WaitTransferComplete<'_, N> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.with_check_and_reset(|inner| {
            if inner.transfer_complete_happened {
                Poll::Ready(())
            } else {
                let new_waker = cx.waker();

                // From embassy
                // https://github.com/embassy-rs/embassy/blob/b99533607ceed225dd12ae73aaa9a0d969a7365e/embassy-sync/src/waitqueue/waker.rs#L59-L61
                match &inner.transfer_complete_waker {
                    // Optimization: If both the old and new Wakers wake the same task, we can simply
                    // keep the old waker, skipping the clone. (In most executor implementations,
                    // cloning a waker is somewhat expensive, comparable to cloning an Arc).
                    Some(w2) if (w2.will_wake(new_waker)) => {}
                    _ => {
                        // clone the new waker and store it
                        if let Some(old_waker) =
                            inner.transfer_complete_waker.replace(new_waker.clone())
                        {
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
                    if let Some(waker) = inner.transfer_complete_waker.take() {
                        waker.wake();
                    }
                }

                Poll::Pending
            }
        })
    }
}

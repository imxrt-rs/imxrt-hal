//! Demonstrates DHCP address assignment, then either TCP or UDP socket communication.
//!
//! The device acquires an IP address using DHCP. This can take a few seconds, so
//! be patient. Plug it into your DHCP-capable router, and wait for it to assign
//! an IP address. Check the device log for information on the assigned IP, or
//! check your router. The LED turns on once DHCP assignment completes.
//!
//! The transport behaviors depend on the `SocketDemo` configuration you select.
//! Change it depending on your desired demo:
//!
//! - `SocketDemo::TcpLoopback` listens on port 5000 for incoming connections.
//!   Once you connect, the socket loops back any data sent to it.
//! - `SocketDemo::UdpBroadcast` occasionally broadcasts UDP packets. The packets
//!   contain a counter that increments for every message sent from your board.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false, dispatchers = [BOARD_SWTASK0])]
mod app {

    /// When using the LPUART backend, you want to occasionally poll
    /// for new log message. This interval (milliseconds) describes how
    /// long you should wait in between polls.
    ///
    /// This constant does nothing when the USBD backend is used, since
    /// the USBD backend uses its own timer for this purpose.
    const LPUART_POLL_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 4;

    /// Variations of this demo.
    ///
    /// See the top-level docs for more information.
    #[allow(dead_code)]
    enum SocketDemo {
        TcpLoopback,
        UdpBroadcast,
    }

    /// Change the demo here.
    const SOCKET_DEMO: SocketDemo = SocketDemo::TcpLoopback;

    use board::smoltcp;
    use hal::enet;
    use imxrt_hal as hal;

    use smoltcp::iface::Config;
    use smoltcp::iface::Interface;
    use smoltcp::iface::SocketSet;
    use smoltcp::socket::dhcpv4;
    use smoltcp::socket::{tcp, udp};
    use smoltcp::time::Instant;
    use smoltcp::wire::EthernetAddress;
    use smoltcp::wire::IpCidr;
    use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint, Ipv4Address};

    #[local]
    struct Local {
        /// This timer tells us how frequently to poll
        /// for logs. It's only used with the LPUART
        /// logging backend.
        poll_log: hal::pit::Pit<1>,
        /// For realizing blocking delays in the idle loop.
        delay: hal::timer::BlockingGpt<1, { board::GPT1_FREQUENCY }>,
        /// The ethernet instance.
        ///
        /// Taken by the idle task.
        enet: Option<imxrt_ral::enet::ENET>,
        led: board::Led,
    }

    #[shared]
    struct Shared {
        /// The poller drives the logging backend.
        poller: board::logging::Poller,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            board::Common {
                pit: (_, mut poll_log, _, _),
                gpt1,
                usb1,
                usbnc1,
                usbphy1,
                mut dma,
                ..
            },
            board::Specifics {
                led, console, enet, ..
            },
        ) = board::new();

        // We only need the extra timer when the LPUART backend is used.
        // The USBD backend uses the USB peripheral's internal timer to
        // track time for us.
        if board::logging::BACKEND == board::logging::Backend::Lpuart {
            poll_log.set_load_timer_value(LPUART_POLL_INTERVAL_MS);
            poll_log.set_interrupt_enable(true);
            poll_log.enable();
        } else {
            poll_log.disable();
        }

        let usbd = hal::usbd::Instances {
            usb: usb1,
            usbnc: usbnc1,
            usbphy: usbphy1,
        };

        let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        let poller = board::logging::init(
            board::logging::Frontend::Log,
            board::logging::BACKEND,
            console,
            dma_a,
            usbd,
        );

        let delay = hal::timer::Blocking::<_, { board::GPT1_FREQUENCY }>::from_gpt(gpt1);

        (
            Shared { poller },
            Local {
                poll_log,
                delay,
                enet: Some(enet),
                led,
            },
            init::Monotonics(),
        )
    }

    #[idle(local = [
        delay, enet, led,
        txdt: enet::TransmitBuffers<2, 1520> = enet::TransmitBuffers::new(),
        rxdt: enet::ReceiveBuffers<2, 1520> = enet::ReceiveBuffers::new(),
        socket_buffer: [u8; 1024] = [0; 1024],
        msg: [u8; 512] = [0; 512],
        udp_tx_meta: [udp::PacketMetadata; 1] = [udp::PacketMetadata::EMPTY],
    ])]
    fn idle(cx: idle::Context) -> ! {
        // Ethernet setup happens here, after init, in order to use interrupt-driven logging.
        let idle::LocalResources {
            enet,
            delay,
            led,
            txdt,
            rxdt,
            socket_buffer,
            msg,
            udp_tx_meta,
        } = cx.local;
        let enet = enet.take().unwrap();

        delay.block_ms(2000);
        let mut dev = enet::Enet::new(
            enet,
            txdt.take(),
            rxdt.take(),
            board::ENET_BUS_FREQUENCY,
            &board::ENET_MAC,
        );
        delay.block_ms(200);

        log::info!("Initializing the PHY, and waiting for link...");
        // Call blocks until the entire link is up! TODO(mciantyre) not block while the link isn't available.
        while let Err(what) = board::init_enet_phy(&mut dev) {
            log::error!("Failed to initialize PHY: {what}");
            delay.block_ms(2000);
        }

        dev.set_duplex(hal::enet::Duplex::Full);
        dev.enable_mib(true);
        dev.enable_rmii_mode(true);
        dev.enable_10t_mode(false);
        dev.enable_mac(true);

        let mut time: i64 = 0;

        let mut iface = Interface::new(
            Config::new(EthernetAddress(board::ENET_MAC).into()),
            &mut dev,
            Instant::from_millis(time),
        );

        let dhcp_socket = dhcpv4::Socket::new();

        let mut sockets: [_; 2] = Default::default();
        let mut sockets = SocketSet::new(sockets.as_mut_slice());
        let dhcp_handle = sockets.add(dhcp_socket);

        log::info!("Waiting for DHCP assignment...");
        loop {
            time += 100;
            delay.block_ms(100);

            iface.poll(Instant::from_millis(time), &mut dev, &mut sockets);
            let dhcp_socket: &mut dhcpv4::Socket = sockets.get_mut(dhcp_handle);
            if let Some(dhcpv4::Event::Configured(config)) = dhcp_socket.poll() {
                led.set();
                log::info!("Received IP assignment! Address is {:?}", config.address);
                iface.update_ip_addrs(|ips| {
                    ips.push(IpCidr::Ipv4(config.address)).unwrap();
                });
                break;
            }
        }

        let msg = msg.as_mut_slice();
        match SOCKET_DEMO {
            SocketDemo::TcpLoopback => {
                let buffer_split = socket_buffer.len() / 2;
                let buffer_splits = socket_buffer.split_at_mut(buffer_split);

                log::info!("Starting TCP loopback mode using port 5000");
                let mut tcp_socket = tcp::Socket::new(
                    tcp::SocketBuffer::new(buffer_splits.0),
                    tcp::SocketBuffer::new(buffer_splits.1),
                );
                tcp_socket.set_nagle_enabled(false);
                if let Err(err) = tcp_socket.listen(IpListenEndpoint {
                    addr: None,
                    port: 5000,
                }) {
                    log::error!("Failed to listen on TCP socket: {err}");
                    panic!();
                }
                let socket_handle = sockets.add(tcp_socket);
                loop {
                    time += 10;
                    delay.block_ms(10);
                    if iface.poll(Instant::from_millis(time), &mut dev, &mut sockets) {
                        let tcp_socket: &mut tcp::Socket = sockets.get_mut(socket_handle);
                        let available = match tcp_socket.recv_slice(msg) {
                            Err(tcp::RecvError::InvalidState) => {
                                log::error!("TCP Receive error: invalid state");
                                continue;
                            }
                            Err(tcp::RecvError::Finished) => {
                                log::warn!("Receive error finished; re-listening on port 5000");
                                tcp_socket.abort();
                                if let Err(err) = tcp_socket.listen(IpListenEndpoint {
                                    addr: None,
                                    port: 5000,
                                }) {
                                    log::error!("Failed to listen on TCP socket: {err}");
                                }
                                continue;
                            }
                            Ok(0) => continue,
                            Ok(n) => n,
                        };

                        log::info!("Received {available} bytes from a client");
                        if let Some(remote) = tcp_socket.remote_endpoint() {
                            log::info!("Remote client: {remote:?}");
                        } else {
                            log::warn!("Not sure of the remote's IP address!");
                        }

                        if let Err(err) = tcp_socket.send_slice(&msg[..available]) {
                            log::error!("TCP send error {err:?}");
                            continue;
                        }
                    }
                }
            }
            SocketDemo::UdpBroadcast => {
                log::info!("Starting UDP broadcast mode");
                let not_receiving = udp::PacketBuffer::new(&mut [][..], &mut [][..]);
                let mut udp_socket = udp::Socket::new(
                    not_receiving,
                    udp::PacketBuffer::new(
                        udp_tx_meta.as_mut_slice(),
                        socket_buffer.as_mut_slice(),
                    ),
                );
                if let Err(err) = udp_socket.bind(IpListenEndpoint {
                    addr: None,
                    port: 5000,
                }) {
                    log::error!("Failed to bind UDP socket: {err}");
                    panic!();
                };
                let socket_handle = sockets.add(udp_socket);
                let mut counter: u8 = 0;
                loop {
                    time += 10;
                    delay.block_ms(10);
                    iface.poll(Instant::from_millis(time), &mut dev, &mut sockets);
                    if time % 1000 == 0 {
                        let udp_socket: &mut udp::Socket = sockets.get_mut(socket_handle);
                        counter = counter.wrapping_add(1);
                        msg.fill(counter);
                        match udp_socket.send_slice(
                            msg,
                            IpEndpoint {
                                addr: IpAddress::Ipv4(Ipv4Address::BROADCAST),
                                port: 5000,
                            },
                        ) {
                            Ok(()) => log::info!("Sent buffer full of counter {counter}"),
                            Err(err) => log::warn!("Failed to send counter {counter}: {err:?}"),
                        }
                    }
                }
            }
        }
    }

    /// This interrupt fires
    ///
    /// - when log messages have been written (to the USB host).
    /// - every few milliseconds; check the imxrt-log docs for the
    ///   specific number.
    #[task(binds = BOARD_USB1, priority = 1)]
    fn usb_interrupt(_: usb_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    /// This interrupt fires
    ///
    /// - when log messages have been written.
    ///
    /// Notice how there's no "periodic" or "time" component here.
    /// When using the LPUART backend, you should use another time
    /// source, or a polling loop, to make sure poll periodically
    /// happens. Without this, you won't see your log messages.
    #[task(binds = BOARD_DMA_A, priority = 1)]
    fn dma_interrupt(_: dma_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    /// Actually performs the poll call.
    #[task(shared = [poller], priority = 2)]
    fn poll_logger(mut cx: poll_logger::Context) {
        cx.shared.poller.lock(|poller| poller.poll());
    }

    /// Periodically runs when using the LPUART logger to drain the log queue.
    #[task(binds = BOARD_PIT, local = [poll_log], priority = 1)]
    fn pit_interrupt(cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources { poll_log } = cx.local;
        while poll_log.is_elapsed() {
            poll_log.clear_elapsed();
        }
        poll_logger::spawn().unwrap();
    }
}

//! Demonstrates DHCP address assignment, then either TCP or UDP socket communication.
//!
//! The device acquires an IP address using DHCP. This can take a few seconds, so
//! be patient. Plug it into your DHCP-capable router, and wait for it to assign
//! an IP address. Check the device defmt for information on the assigned IP, or
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
        /// For realizing blocking delays in the idle loop.
        delay: hal::timer::BlockingGpt<1, { board::GPT1_FREQUENCY }>,
        /// The ethernet instance.
        ///
        /// Taken by the idle task.
        enet: Option<imxrt_ral::enet::ENET>,
        led: board::Led,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (board::Common { gpt1, .. }, board::Specifics { led, enet, .. }) = board::new();

        let delay = hal::timer::Blocking::<_, { board::GPT1_FREQUENCY }>::from_gpt(gpt1);

        (
            Shared {},
            Local {
                delay,
                enet: Some(enet),
                led,
            },
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
        //
        // TODO(mciantyre) that's not necessary if we use defmt_rtt.
        let idle::LocalResources {
            enet,
            delay,
            led,
            txdt,
            rxdt,
            socket_buffer,
            msg,
            udp_tx_meta,
            ..
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

        defmt::info!("Initializing the PHY, and waiting for link...");
        // Call blocks until the entire link is up! TODO(mciantyre) not block while the link isn't available.
        while let Err(what) = board::init_enet_phy(&mut dev) {
            defmt::error!("Failed to initialize PHY: {}", what);
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

        defmt::info!("Waiting for DHCP assignment...");
        loop {
            time += 100;
            delay.block_ms(100);

            iface.poll(Instant::from_millis(time), &mut dev, &mut sockets);
            let dhcp_socket: &mut dhcpv4::Socket = sockets.get_mut(dhcp_handle);
            if let Some(dhcpv4::Event::Configured(config)) = dhcp_socket.poll() {
                led.set();
                defmt::info!("Received IP assignment! Address is {:?}", config.address);
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

                defmt::info!("Starting TCP loopback mode using port 5000");
                let mut tcp_socket = tcp::Socket::new(
                    tcp::SocketBuffer::new(buffer_splits.0),
                    tcp::SocketBuffer::new(buffer_splits.1),
                );
                tcp_socket.set_nagle_enabled(false);
                if let Err(err) = tcp_socket.listen(IpListenEndpoint {
                    addr: None,
                    port: 5000,
                }) {
                    defmt::error!("Failed to listen on TCP socket: {}", err);
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
                                defmt::error!("TCP Receive error: invalid state");
                                continue;
                            }
                            Err(tcp::RecvError::Finished) => {
                                defmt::warn!("Receive error finished; re-listening on port 5000");
                                tcp_socket.abort();
                                if let Err(err) = tcp_socket.listen(IpListenEndpoint {
                                    addr: None,
                                    port: 5000,
                                }) {
                                    defmt::error!("Failed to listen on TCP socket: {}", err);
                                }
                                continue;
                            }
                            Ok(0) => continue,
                            Ok(n) => n,
                        };

                        defmt::info!("Received {} bytes from a client", available);
                        if let Some(remote) = tcp_socket.remote_endpoint() {
                            defmt::info!("Remote client: {:?}", remote);
                        } else {
                            defmt::warn!("Not sure of the remote's IP address!");
                        }

                        if let Err(err) = tcp_socket.send_slice(&msg[..available]) {
                            defmt::error!("TCP send error {:?}", err);
                            continue;
                        }
                    }
                }
            }
            SocketDemo::UdpBroadcast => {
                defmt::info!("Starting UDP broadcast mode");
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
                    defmt::error!("Failed to bind UDP socket: {}", err);
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
                            Ok(()) => defmt::info!("Sent buffer full of counter {}", counter),
                            Err(err) => {
                                defmt::warn!("Failed to send counter {}: {:?}", counter, err)
                            }
                        }
                    }
                }
            }
        }
    }
}

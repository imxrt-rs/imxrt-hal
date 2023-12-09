use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::args()
        .skip(1)
        .next()
        .ok_or("Provide an IPv4 address")?;

    let ip: std::net::Ipv4Addr = ip.parse()?;

    let mut client = TcpStream::connect(std::net::SocketAddrV4::new(ip, 5000))?;

    const MSG: &[u8] = b"Hello, world!";
    client.write(MSG)?;
    let mut resp = [0; MSG.len()];
    client.read(&mut resp)?;

    if resp == MSG {
        Ok(())
    } else {
        Err(format!("Expected '{MSG:?}' but received '{resp:?}'").into())
    }
}

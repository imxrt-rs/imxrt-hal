use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::args()
        .skip(1)
        .next()
        .ok_or("Provide an IPv4 address")?;

    let ip: std::net::Ipv4Addr = ip.parse()?;

    let mut client = TcpStream::connect(std::net::SocketAddrV4::new(ip, 5000))?;
    client.set_nodelay(true)?;

    const MSG: &[u8] = b"Hello, world!";
    client.write(MSG)?;
    let mut resp = [0; MSG.len()];
    client.read(&mut resp)?;

    if resp != MSG {
        return Err(format!("Expected '{MSG:?}' but received '{resp:?}'").into());
    }

    let mut resp = vec![0; 256];
    for idx in 0..10 {
        let msg: Vec<u8> = Vec::from_iter((0..=255u8).cycle().skip(idx).take(resp.len()));
        client.write(&msg)?;
        client.read(&mut resp)?;
        if msg != resp {
            return Err(format!("Mismatched buffer on {idx} attempt").into());
        }
    }

    Ok(())
}

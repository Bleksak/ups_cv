use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::str;

fn connect() -> Result<TcpStream, io::Error> {
    let mut args = std::env::args();
    args.next().unwrap();
    let addr = args.next().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Please provide IP address",
    ))?;

    let port = args.next().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Please provide a port",
    ))?;

    port.parse::<u16>()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid port"))
        .and_then(|port| match port {
            0 => Err(io::Error::new(io::ErrorKind::Other, "Port 0 is forbidden")),
            _ => Ok(TcpStream::connect(format!("{}:{}", addr, port))),
        })?
}

fn main() -> Result<(), io::Error> {
    let mut stream = connect()?;
    stream.set_read_timeout(Some(std::time::Duration::from_millis(300)))?;

    loop {
        let mut buf = [0; 512];
        let result = stream.read(&mut buf);

        match result {
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock => break,
                _ => {}
            },
            _ => {}
        }

        println!(
            "{}",
            str::from_utf8(&buf)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Received invalid string"))?
        );
    }
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    stream.write_all(&line.into_bytes())?;

    let mut res = String::new();
    stream.read_to_string(&mut res)?;
    println!("{}", res);
    Ok(())
}

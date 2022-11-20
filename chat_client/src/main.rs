use std::net::TcpStream;
use std::io::{self, Read, Write, BufRead};

fn read_all(client: &mut TcpStream) -> Option<Vec<u8>> {
    let mut data: Vec<u8> = Vec::with_capacity(512);
    data.resize(512, 0);

    let mut read_bytes = 0;

    let read = client.read(&mut data[..]);

    if let Err(err) = read {
        match err.kind() {
            io::ErrorKind::WouldBlock => {
                return None;
            }

            _ => {}
        }
    } else {
        read_bytes += read.unwrap();
    }

    loop {
        data.resize(read_bytes + 512, 0);
        let read = client.read(&mut data[read_bytes..read_bytes + 512]);

        if let Err(err) = read {
            match err.kind() {
                io::ErrorKind::WouldBlock => {
                    data.truncate(read_bytes);
                    return Some(data);
                }

                _ => {
                    return None;
                }
            }
        } else {
            let read = read.unwrap();
            if read == 0 {
                data.truncate(read_bytes);
                return Some(data);
            }
            read_bytes += read;
        }
    }
}


fn main() -> Result<(), io::Error> {
    let mut client_socket = TcpStream::connect("127.0.0.1:2000")?;
    client_socket.set_nonblocking(true).expect("Failed to set nonblocking");
    let mut client_clone = client_socket.try_clone()?;
    let mut username = String::new();
    
    print!("Enter your username: ");
    io::stdout().lock().flush()?;
    
    io::stdin().lock().read_line(&mut username).expect("Invalid username");
    username = username.trim().to_owned();
    
    std::thread::spawn(move|| {
        loop {
            if let Some(mut data) = read_all(&mut client_clone) {
                data.push('\n' as u8);
                if let Err(_) = io::stdout().lock().write_all(&data) {
                    println!("Corrupted message");
                }
            }
        }
    });
    
    loop {
        let mut buf = String::new();
        if let Ok(_) = io::stdin().lock().read_line(&mut buf) {
            buf = buf.trim().to_owned();
            
            let mut message = String::new();
            message.push_str(&username);
            message.push_str(": ");
            message.push_str(&buf);
            
            if let Err(err) = client_socket.write_all(&message.into_bytes()) {
                println!("Failed to send a message: {}", err);
            }
        }
    }
}

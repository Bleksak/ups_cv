use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::prelude::AsRawFd;
use std::str;
use std::sync::mpsc::{self, Receiver, Sender};

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
#[cfg(target_os = "windows")]
fn sock_fd(stream: &TcpStream) -> i32 {
    stream.as_raw_socket() as i32
}

#[cfg(target_os = "linux")]
fn sock_fd(stream: &TcpStream) -> i32 {
    stream.as_raw_fd()
}

fn main() -> Result<(), io::Error> {
    let server_socket = TcpListener::bind("127.0.0.1:2000")?;
    server_socket
        .set_nonblocking(true)
        .expect("cannot set nonblocking");

    let server_socket_clone = server_socket.try_clone()?;
    let (sender, receiver): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();

    let mut clients = vec![];

    std::thread::spawn(move || {
        for client in server_socket_clone.incoming() {
            if let Ok(client) = client {
                sender.send(client).unwrap();
            }
            
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    });

    loop {
        while let Ok(client) = receiver.try_recv() {
            println!("got client!");
            client
                .set_nonblocking(true)
                .expect("cannot set nonblocking client");
            clients.push(client);
        }

        let mut disconnect = vec![];
        let mut ignore = vec![];
        let mut broadcast = vec![];

        for (index, client) in clients.iter_mut().enumerate() {
            if let Some(data) = read_all(client) {
                println!("read");
                if data.len() == 0 {
                    println!("disconnect");
                    disconnect.push(index);
                } else {
                    if let Ok(message) = str::from_utf8(&data) {
                        println!("{}", message);
                        ignore.push(sock_fd(client));
                        broadcast.push(data);
                    }
                }
            }
        }

        for client in disconnect.iter().rev() {
            clients.remove(*client);
        }

        for (message,ignore) in broadcast.iter().zip(ignore.iter()) {
            for client in clients.iter_mut().filter(|a | a.as_raw_fd() != *ignore) {
                if let Err(err) = client.write_all(&message) {
                    println!("Failed to send message to client: {}", err);
                }
            }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}


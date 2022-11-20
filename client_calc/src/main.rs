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

fn get_op(c: Option<char>) -> Option<String> {
    if let Some(c) = c {
        match c {
            '+' => Some("plus".to_string()),
            '-' => Some("minus".to_string()),
            '*' => Some("multiply".to_string()),
            '/' => Some("division".to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn get_number() -> Option<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).ok()?;
    line.trim().parse::<i64>().ok()
}

fn main() -> Result<(), io::Error> {
    loop {
        let mut stream = connect()?;
        println!("Input operator and two numbers (plus minus multiply division) (divided by |)");
        
        let num1;
        let num2;
        
        loop {
            if let Some(num) = get_number() {
                num1 = num;
                break;
            } else {
                println!("Invalid input (integer)!");
            }
        };
        
        loop {
            if let Some(num) = get_number() {
                num2 = num;
                break;
            } else {
                println!("Invalid input (integer)!");
            }
        };
        
        let op;
        
        loop {
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            
            if line.len() > 2 {
                continue;
            }
            
            if let Some(operation) = get_op(line.trim().chars().next()) {
                op = operation;
                break;
            } else {
                println!("Invalid input! (+-/*)");
            }
        }
        
        let message = format!("{}|{}|{}\n", op, num1, num2);
        stream.write_all(&message.into_bytes())?;
        
        let mut res = String::new();
        stream.read_to_string(&mut res)?;
        println!("{}", res);
    }
}

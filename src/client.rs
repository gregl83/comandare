use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

use crate::Logger;
use crate::format_address;

pub fn connect(log: Logger, host: &str, port: u16) {
    let address = format_address(host, port);
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            log.out(format!("Successfully connected to server in port {}", port));

            let msg = b"Hello!";

            stream.write(msg).unwrap();

            log.out(format!("Sent Hello, awaiting reply..."));

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        log.out(format!("Reply is ok!"));
                    } else {
                        let text = from_utf8(&data).unwrap();
                        log.out(format!("Unexpected reply: {}", text));
                    }
                },
                Err(e) => {
                    log.out(format!("Failed to receive data: {}", e));
                }
            }
        },
        Err(e) => {
            log.out(format!("Failed to connect: {}", e));
        }
    }
    log.out(format!("Terminated."));
}
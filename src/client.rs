use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

use crate::Logger;
use crate::format_address;

pub fn connect(log: Logger, host: &str, port: u16) {
    let address = format_address(host, port);
    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            log.out(format!("Successfully connected to server in port {}", port));

            let msg = b"ls";

            let mut buffer = Vec::new();

            stream.write_all(msg).unwrap();
            stream.shutdown(Shutdown::Write).unwrap();

            log.out(format!("Sent command to {}", &address));

            match stream.read_to_end(&mut buffer) {
                Ok(_) => {
                    let response = String::from_utf8( buffer ).unwrap();
                    // todo - add always out feature
                    println!("{}", response);
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
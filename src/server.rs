use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

use crate::{
    Logger,
    Loggable,
    format_address,
    parse
};

fn handle_client(log: Logger, mut stream: TcpStream) {
    let mut buffer = Vec::new();
    match stream.read_to_end(&mut buffer) {
        Ok(_) => {
            let command = String::from_utf8( buffer ).unwrap();
            log.out(format!("Command received: {}", command));
            let result = parse(&command);
            match result {
                Ok(output) => {
                    stream.write_all(output.stdout.as_slice()).unwrap();
                    stream.write_all(output.stderr.as_slice()).unwrap();
                }
                Err(e) => {
                    log.out(format!("An error occurred, terminating connection with: {}", e));
                }
            }
            stream.shutdown(Shutdown::Both).unwrap();
        },
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            log.out(format!("An error occurred, terminating connection with: {}", stream.peer_addr().unwrap()));
        }
    }
}

pub fn listen(log: Logger, host: &str, port: u16) {
    let address = format_address(host, port);
    let listener = TcpListener::bind(address).unwrap();

    log.out(format!("Server listening on port {}", port));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log.out(format!("New connection: {}", stream.peer_addr().unwrap()));

                let thread_log = log.clone();
                thread::spawn(move|| {
                    handle_client(thread_log, stream);
                });
            }
            Err(e) => {
                log.out(format!("Error: {}", e));
            }
        }
    }
}
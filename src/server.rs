use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

use crate::Logger;
use crate::format_address;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            // fixme - use logger
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn listen(log: Logger, host: &str, port: u16) {
    let address = format_address(host, port);
    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    log.out(format!("Server listening on port {}", port));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log.out(format!("New connection: {}", stream.peer_addr().unwrap()));
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                log.out(format!("Error: {}", e));
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
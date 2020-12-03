use std::env;
use std::process::exit;
use comandare::{listen, connect};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("comandare [mode] [host] [port]");
        exit(1);
    }

    let host = &args[2];
    let port = &args[3];
    let port: u16 = match port.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    match args[1].as_str() {
        "server" => {
            listen(host, port);
        }
        "client" => {
            connect(host, port);
        },
        _ => {
            eprintln!("Run as mode of 'server' or 'client'.");
            exit(1);
        }
    }
}

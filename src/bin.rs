use std::env;
use std::process::exit;
use comandare::{
    Logger,
    listen,
    connect
};

const MODES: [&str; 2] = [
    "client",
    "server"
];

const DEBUG_FLAGS: [&str; 2] = [
    "--debug",
    "-d"
];

fn main() {
    let args: Vec<String> = env::args().collect();

    // check number of arguments
    if args.len() < 3 {
        eprintln!("[MODE] [HOST] [PORT] [:--DEBUG|-D] -- [:COMMAND]");
        exit(1);
    }

    // check for valid mode argument
    let mode = &args[1];
    if !MODES.contains(&mode.as_str()) {
        eprintln!("MODE must be 'client' or 'server'");
        exit(1);
    }

    let host = &args[2];

    // check for valid u16 port number
    let port = &args[3];
    let port: u16 = match port.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    // check for debug switch
    let debug = args.iter().find(|argument| {
        DEBUG_FLAGS.contains(&argument.to_lowercase().as_str())
    });

    // implement logger with basic debug switch
    let log = Logger::new(debug.is_some());

    match mode.as_str() {
        "server" => {
            listen(log, host, port);
        }
        "client" => {
            match args.iter().position(|x| x.eq("--")) {
                Some(index) => {
                    let command_offset = index + 1;
                    let command: &str = &args[command_offset..args.len()].join(" ");
                    connect(log, host, port, command);
                }
                None => {
                    eprintln!("MODE 'client' requires COMMAND");
                }
            }
        },
        _ => {}
    }
}

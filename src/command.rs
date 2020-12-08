use std::io;
use std::process::{Command, Output};

pub fn parse(command: &str) -> io::Result<Output> {
    // parse command for program and args
    let args: Vec<_> = command.split(" ").collect();
    let result = Command::new(args[0]).args(&args[1..]).output();
    result
}
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Logger {
    debug: bool
}

impl Logger {
    pub fn new(debug: bool) -> Self {
        Logger {
            debug,
        }
    }

    pub fn out(&self, message: String) {
        if self.debug {
            io::stdout().write_all(message.as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
    }

    pub fn err(&self, message: String) {
        io::stderr().write_all(message.as_bytes());
        io::stderr().flush().unwrap();
    }
}
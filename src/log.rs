use std::io::{self, Write};

pub trait Loggable {
    fn out(&self, message: String);
    fn err(&self, message: String);
}

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
}

impl Loggable for Logger {
    fn out(&self, message: String) {
        if self.debug {
            io::stdout().write_all(message.as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
    }

    fn err(&self, message: String) {
        io::stderr().write_all(message.as_bytes()).unwrap();
        io::stderr().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_out() {
        assert_eq!(1, 2);
    }

    #[test]
    fn logs_err() {
        assert_eq!(1, 2);
    }
}
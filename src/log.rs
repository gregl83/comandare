use std::io::Write;
use std::sync::{Arc, Mutex};

pub trait Loggable {
    fn out(&mut self, message: String);
    fn err(&mut self, message: String);
}

pub struct Logger {
    debug: bool,
    stdout: Arc<Mutex<dyn Write + Send>>,
    stderr: Arc<Mutex<dyn Write + Send>>,
}

impl Logger {
    pub fn new(debug: bool, stdout: Arc<Mutex<dyn Write + Send>>, stderr: Arc<Mutex<dyn Write + Send>>) -> Self {
        Logger {
            debug,
            stdout,
            stderr,
        }
    }
}

impl Loggable for Logger {
    fn out(&mut self, message: String) {
        if self.debug {
            let mut stdout = self.stdout.lock().unwrap();
            stdout.write_all(message.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }
    }

    fn err(&mut self, message: String) {
        let mut stderr = self.stderr.lock().unwrap();
        stderr.write_all(message.as_bytes()).unwrap();
        stderr.flush().unwrap();
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Logger {
            debug: self.debug,
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
        }
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
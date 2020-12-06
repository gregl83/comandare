use std::io::Write;
use std::sync::{Arc, Mutex};

type LogStream = Arc<Mutex<dyn Write + Send>>;

pub trait Loggable {
    fn out(&mut self, message: String);
    fn err(&mut self, message: String);
}

pub struct Logger {
    debug: bool,
    stdout: LogStream,
    stderr: LogStream,
}

impl Logger {
    pub fn new(debug: bool, stdout: LogStream, stderr: LogStream) -> Self {
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
    use mockall::mock;
    use std::io::Result;

    mock!{
        Stdout {}
        trait Write{
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn write_all(&mut self, mut buf: &[u8]) -> Result<()>;
            fn flush(&mut self) -> Result<()>;
        }
    }

    mock!{
        Stderr {}
        trait Write{
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn write_all(&mut self, mut buf: &[u8]) -> Result<()>;
            fn flush(&mut self) -> Result<()>;
        }
    }

    #[test]
    fn logs_out() {
        let expected = format!("info");

        let mut stdout_mock = MockStdout::new();
        // todo - add string eq predicate
        stdout_mock.expect_write_all()
            .times(1)
            .returning(|_| Ok(()));
        stdout_mock.expect_flush()
            .times(1)
            .returning(|| Ok(()));
        let stderr_mock = MockStderr::new();
        let mut logger = Logger::new(
            true,
            Arc::new(Mutex::new(stdout_mock)),
            Arc::new(Mutex::new(stderr_mock))
        );
        logger.out(expected.clone());
    }

    #[test]
    fn logs_err() {
        let expected = format!("error");

        let stdout_mock = MockStdout::new();
        let mut stderr_mock = MockStderr::new();
        // todo - add string eq predicate
        stderr_mock.expect_write_all()
            .times(1)
            .returning(|_| Ok(()));
        stderr_mock.expect_flush()
            .times(1)
            .returning(|| Ok(()));
        let mut logger = Logger::new(
            true,
            Arc::new(Mutex::new(stdout_mock)),
            Arc::new(Mutex::new(stderr_mock))
        );
        logger.err(expected.clone());
    }
}
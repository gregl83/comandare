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
            println!("{}", message);
        }
    }
}
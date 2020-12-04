mod log;
mod client;
mod server;
mod command;

// expose interface
pub use log::{Logger, Loggable};
pub use client::connect;
pub use server::listen;
pub use command::parse;

fn format_address(host: &str, port: u16) -> String {
    format!("{}:{}", host, port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_network_address() {
        assert_eq!("localhost:0", format_address("localhost", 0));
        assert_eq!("localhost:1234", format_address("localhost", 1234));

        assert_eq!("0.0.0.0:0", format_address("0.0.0.0", 0));
        assert_eq!("0.0.0.0:1234", format_address("0.0.0.0", 1234));

        assert_eq!("127.0.0.1:0", format_address("127.0.0.1", 0));
        assert_eq!("127.0.0.1:1234", format_address("127.0.0.1", 1234));
    }
}

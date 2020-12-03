mod log;
mod client;
mod server;

// expose interface
pub use log::Logger;
pub use client::connect;
pub use server::listen;

fn format_address(host: &str, port: u16) -> String {
    format!("{}:{}", host, port)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

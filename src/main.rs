use std::net::TcpStream;
use std::time::Duration;
fn main() {
    let target = "127.0.0.1";

    for port in 1..=100{
        let address = format!("{}:{}", target, port);
        match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_millis(100)) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => println!("Port {} is closed", port),
        } 
    }
}
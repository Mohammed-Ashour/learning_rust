use std::net::TcpStream;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();

    let data = "Hello, world!\r\n";
    stream.write(data.as_bytes()).unwrap();
}
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    let n = stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer[..n]).unwrap()
    );
}

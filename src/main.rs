mod mail;
mod commands;
mod request;

use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;

fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:4450").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(stream);

        let mut string = String::new();
        reader.read_line(&mut string).unwrap();

        println!("{:02X?}", string.as_bytes());

    }
}

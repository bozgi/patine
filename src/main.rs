mod mail;
mod commands;
mod request;

use std::io::{BufRead, Read};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:4450").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            todo!("Handle requests")
        });
    }
}

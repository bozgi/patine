use bytes::BytesMut;
use tokio::io::BufWriter;
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut
}

enum ConnectionFrame {
    
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Self {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(1024),
        }
    }



}
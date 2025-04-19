use tokio::net::TcpStream;

pub struct Request {
    stream: TcpStream
}

impl Request {
    pub fn handle_stream(mut stream: TcpStream) {
        todo!("Implement")
    }
}
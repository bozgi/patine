use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec, FramedWrite, LinesCodecError, Decoder};

pub async fn handle_request(mut tcp_stream: TcpStream) {
    tcp_stream.write(b"220 Hello! listening for commands - patine").await.unwrap(); //
    tcp_stream.read(&mut [0; 256]).await.unwrap();
}

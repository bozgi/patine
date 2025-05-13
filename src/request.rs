use std::io::Error;
use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec, FramedWrite, LinesCodecError, Decoder};
use tracing::{debug, info};
use crate::commands::smtp_codec::SmtpCodec;
use crate::commands::smtp_command::SmtpCommand;
use crate::commands::smtp_response::SmtpResponse;

pub async fn handle_request(mut tcp_stream: TcpStream) {
    let mut frame = Framed::new(tcp_stream, SmtpCodec::new());

    frame.send(SmtpResponse::SingleLine(220, "Prototype Patine build".to_string())).await.unwrap();
    frame.flush().await.unwrap();

    while let Some(result) = frame.next().await {
        info!("Handling {:?}", result);
        match result {
            Ok(command) => {
                info!("Frame received: {:?}", command);
                match command {
                    SmtpCommand::Ehlo(ref a) => {
                        frame.send(SmtpResponse::Multiline(250, vec![
                            "Welcome!".to_string(),
                            "SMTPUTF8".to_string(),
                        ])).await.unwrap();
                    }
                    SmtpCommand::Quit => {
                        frame.send(SmtpResponse::SingleLine(221, "OK".to_string())).await.unwrap();
                        return;
                    }
                    SmtpCommand::Unknown => {
                        frame.send(SmtpResponse::SingleLine(500, "Unknown command".to_string())).await.unwrap();
                    }
                    _ => {
                        frame.send(SmtpResponse::SingleLine(502, "Command not implemented".to_string())).await.unwrap();
                    }
                }
            }
            Err(_) => {
            }
        }
    }

}

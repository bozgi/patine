use crate::command::smtp_command::SmtpCommand;
use crate::io::codec_state::CodecState;
use crate::io::smtp_response::SmtpResponse;
use bytes::{BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use tracing::{debug, trace};

pub struct SmtpCodec {
    state: CodecState
}

impl SmtpCodec {
    pub fn new() -> SmtpCodec {
        Self {
            state: CodecState::Regular
        }
    }
}

impl Decoder for SmtpCodec {
    type Item = SmtpCommand;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        trace!("Decoding SmtpCodec with the current size {:?}", src.len());
        if let Some(position) = src.windows(2).position(|window| window == b"\r\n") {
            let line = src.split_to(position + 2);
            let line = String::from_utf8_lossy(&line[..line.len() - 2]);

            Ok(Some(SmtpCommand::from(line.to_string())))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<SmtpResponse> for SmtpCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: SmtpResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            SmtpResponse::SingleLine(code, message) => {
                let line = format!("{} {}\r\n", code, message);
                dst.extend_from_slice(line.as_bytes());
            }
            SmtpResponse::Multiline(code, lines) => {
                for (i, line) in lines.iter().enumerate() {
                    let sep = if i == lines.len() - 1 { " " } else { "-" };
                    let line = format!("{}{}{}\r\n", code, sep, line);
                    dst.extend_from_slice(line.as_bytes());
                }
            }
        }
        Ok(())
    }
}
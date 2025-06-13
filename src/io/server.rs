use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};
use tracing::trace;
use crate::command::smtp_command::SmtpCommand;
use crate::io::codec_state::CodecState;
use crate::io::smtp_codec::SmtpCodec;
use crate::io::smtp_response::SmtpResponse;

impl Decoder for SmtpCodec<SmtpCommand, SmtpResponse> {
    type Item = SmtpCommand;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        trace!("Decoding SmtpCodec with the current size {:?}", src.len());
        match &self.codec_state {
            CodecState::Regular => {
                if let Some(position) = src.windows(2).position(|window| window == b"\r\n") {
                    let line = src.split_to(position + 2);
                    let line = String::from_utf8_lossy(&line[..line.len() - 2]);

                    let command = Some(SmtpCommand::from(line.to_string()));

                    trace!("Found command {:?}", command);

                    if let Some(SmtpCommand::Data) = &command {
                        self.codec_state = CodecState::Data;
                    }

                    Ok(command)
                } else {
                    Ok(None)
                }
            }
            CodecState::Data => {
                if let Some(position) = src.windows(5).position(|window| window == b"\r\n.\r\n") {
                    let mail = src.split_to(position + 5);
                    let mail_bytes = mail[..mail.len() - 5].to_vec();

                    self.codec_state = CodecState::Regular;
                    Ok(Some(SmtpCommand::DataEnd(mail_bytes)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl Encoder<SmtpResponse> for SmtpCodec<SmtpCommand, SmtpResponse> {
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
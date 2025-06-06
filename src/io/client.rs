use std::io::{Error, ErrorKind, Read};
use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use crate::command::smtp_command::SmtpCommand;
use crate::io::codec_state::CodecState;
use crate::io::smtp_codec::SmtpCodec;
use crate::io::smtp_response::SmtpResponse;

impl Decoder for SmtpCodec<SmtpResponse, SmtpCommand> {
    type Item = SmtpResponse;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let crlf_pos = src.windows(2).position(|w| w == b"\r\n");
        if crlf_pos.is_none() {
            return Ok(None);
        }

        let pos = crlf_pos.unwrap();
        let line = src.split_to(pos + 2);
        let line_str = std::str::from_utf8(&line[..line.len() - 2])
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8"))?
            .to_string();

        if line_str.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Short line"));
        }

        let code: u16 = line_str[0..3].parse()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid response code"))?;
        let is_last = &line_str[3..4] == " ";

        let message = line_str[4..].trim_start().to_string();

        match &mut self.codec_state {
            CodecState::Regular { buffer: Some(buffer) } => {
                if is_last && buffer.is_empty() {
                    Ok(Some(SmtpResponse::SingleLine(code, message)))
                } else if is_last {
                    buffer.push(message);
                    let lines = std::mem::take(buffer);
                    Ok(Some(SmtpResponse::Multiline(code, lines)))
                } else {
                    buffer.push(message);
                    Ok(None)
                }
            }
            CodecState::Data => {
                Err(Error::new(ErrorKind::InvalidData, "Unexpected response during DATA state"))
            }
            _ => {
                Ok(None)
            }
        }
    }
}

impl Encoder<SmtpCommand> for SmtpCodec<SmtpResponse, SmtpCommand> {
    type Error = Error;

    fn encode(&mut self, item: SmtpCommand, dst: &mut BytesMut) -> Result<(), Self::Error> {
        use SmtpCommand::*;

        let line = match item {
            Helo(domain) => format!("HELO {}\r\n", domain),
            Ehlo(domain) => format!("EHLO {}\r\n", domain),
            Mail(from) => format!("MAIL FROM:<{}>\r\n", from),
            Rcpt(to) => format!("RCPT TO:<{}>\r\n", to),
            Data => "DATA\r\n".to_string(),
            Noop => "NOOP\r\n".to_string(),
            Quit => "QUIT\r\n".to_string(),
            Rset => "RSET\r\n".to_string(),
            Vrfy(param) => format!("VRFY {}\r\n", param),
            Starttls => "STARTTLS\r\n".to_string(),
            Auth(auth_type) => format!("AUTH {}\r\n", auth_type),
            DataEnd(content) => {
                dst.extend_from_slice(&content);

                return Ok(())
            },
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid SMTP command")),
        };

        dst.put(line.as_bytes());
        Ok(())
    }
}
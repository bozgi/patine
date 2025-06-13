use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_codec::SmtpCodec;
use crate::io::smtp_response::SmtpResponse;
use bytes::{Buf, BufMut, BytesMut};
use std::io::{Error, ErrorKind};
use tokio_util::codec::{Decoder, Encoder};
use tracing::trace;

impl Decoder for SmtpCodec<SmtpResponse, SmtpCommand> {
    type Item = SmtpResponse;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut lines = Vec::new();
        let mut total_consumed = 0;
        let mut code: Option<u16> = None;

        loop {
            if let Some(pos) = src[total_consumed..].windows(2).position(|w| w == b"\r\n") {
                let line_end = total_consumed + pos;
                let line = src[total_consumed..line_end].to_vec();
                total_consumed = line_end + 2;

                let line_str = String::from_utf8_lossy(&line);
                trace!("Line: {}", line_str);

                let this_code = line_str[0..3].parse::<u16>().map_err(|_| {
                    Error::new(ErrorKind::InvalidData, "Invalid status code")
                })?;

                let sep = line_str.as_bytes()[3];
                let message = line_str[4..].to_string();

                if code.is_none() {
                    code = Some(this_code);
                } else if code != Some(this_code) {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Inconsistent multiline status codes",
                    ));
                }

                lines.push(message);

                if sep == b' ' {
                    src.advance(total_consumed);
                    let code = code.unwrap();

                    return if lines.len() == 1 {
                        Ok(Some(SmtpResponse::SingleLine(code, lines.remove(0))))
                    } else {
                        Ok(Some(SmtpResponse::Multiline(code, lines)))
                    }
                } else if sep != b'-' {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Malformed response separator",
                    ));
                }
            } else {
                break;
            }
        }

        Ok(None)
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
                dst.extend_from_slice(b"\r\n.\r\n");

                return Ok(())
            },
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid SMTP command")),
        };

        dst.put(line.as_bytes());
        Ok(())
    }
}
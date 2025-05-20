use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_codec::SmtpCodec;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::trace;
use crate::command::handlers::registry::HANDLERS;

pub struct SmtpTransaction {
    pub tls: bool,
    pub esmtp: bool,
    pub state: SmtpState,
    pub from: Option<String>,
    pub to: Option<Vec<String>>,
    framed: Framed<TcpStream, SmtpCodec>,
}

impl SmtpTransaction {
    pub fn new(tcp_stream: TcpStream) -> SmtpTransaction {
        Self {
            tls: false,
            esmtp: false,
            state: SmtpState::Connected,
            from: None,
            to: None,
            framed: Framed::new(tcp_stream, SmtpCodec::new()),
        }
    }

    pub async fn handle_connection(&mut self) {
        self.send_line(220, String::from("Welcome! Patine build 0.1-dev")).await;
        while let Some(Ok(command)) = self.framed.next().await {
            if let Some(command_name) = command.name() {
                if let Some(handler) = HANDLERS.get(&command_name) {
                    handler.handle(self, command).await;
                }
            } else {
                self.send_line(500, String::from("Invalid command")).await;
            }

            if let SmtpState::Finished = self.state {
                break;
            }
        }

        trace!("SmtpTransaction connection closed");
    }

    async fn handle_connected_state(&mut self, command: SmtpCommand) {
        trace!("Handling connected state for command: {:?}", command);
        match command {
            SmtpCommand::Helo(_) => {
                self.state = SmtpState::Greeted;
                self.send_line(250, String::from("Welcome!")).await;
            }
            SmtpCommand::Ehlo(_) => {
                self.state = SmtpState::Greeted;
                self.esmtp = true;
                self.send_line(250, String::from("Welcome!")).await;
            }
            SmtpCommand::Rset => {
                self.state = SmtpState::Connected;
                self.from = None;
                self.to = None;
                self.esmtp = false;
                self.send_line(250, String::from("OK")).await;
            }
            SmtpCommand::Noop => {
                self.send_line(250, String::from("NOOP")).await;
            }
            SmtpCommand::Quit => {
                self.send_line(250, String::from("Goodbye!")).await;
                return;
            }
            SmtpCommand::Vrfy(_) => {
                self.send_line(250, String::from("NOOP")).await;
            }
            SmtpCommand::Unknown => {
                self.send_line(500, String::from("Unknown command")).await;
            }
            _ => {
                self.send_line(503, String::from("Bad sequence")).await;
            }
        }
    }

    pub async fn send_line(&mut self, code: u16, message: String) {
        self.framed.send(SmtpResponse::SingleLine(code, message))
            .await
            .unwrap();
    }

    pub async fn send_multiline(&mut self, code: u16, message: Vec<String>) {
        self.framed.send(SmtpResponse::Multiline(code, message))
            .await
            .unwrap();
    }
}

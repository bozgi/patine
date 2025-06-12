use crate::command::handlers::registry::HANDLERS;
use crate::command::smtp_command::SmtpCommand;
use crate::io::dns::RESOLVER;
use crate::io::smtp_codec::SmtpCodec;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::tls::{ACCEPTOR};
use crate::io::transaction_type::TransactionType;
use futures::{SinkExt, StreamExt};
use std::io::{Error, ErrorKind};
use std::net::ToSocketAddrs;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::{trace, warn};
use crate::command::smtp_command::SmtpCommand::Quit;
pub(crate) use crate::io::asyncio::AsyncIO;
use crate::storage::maildir::DOMAIN;

pub struct SmtpTransaction<I, O> {
    pub tls: bool,
    pub esmtp: bool,
    pub authenticated: bool,
    pub state: SmtpState,
    pub from: Option<String>,
    pub to: Option<Vec<String>>,
    pub transaction_type: TransactionType,
    framed: Framed<Box<dyn AsyncIO + Unpin + Send>, SmtpCodec<I, O>>,
}

impl SmtpTransaction<SmtpCommand, SmtpResponse> {
    pub fn new_server(tcp_stream: TcpStream) -> SmtpTransaction<SmtpCommand, SmtpResponse> {
        Self {
            tls: false,
            esmtp: false,
            authenticated: true,
            state: SmtpState::Connected,
            from: None,
            to: None,
            transaction_type: TransactionType::SERVER,
            framed: Framed::new(Box::new(tcp_stream), SmtpCodec::new()),
        }
    }

    pub fn new_submission(tcp_stream: TcpStream) -> SmtpTransaction<SmtpCommand, SmtpResponse> {
        Self {
            tls: false,
            esmtp: false,
            authenticated: false,
            state: SmtpState::Connected,
            from: None,
            to: None,
            transaction_type: TransactionType::SUBMISSION,
            framed: Framed::new(Box::new(tcp_stream), SmtpCodec::new()),
        }
    }

    pub async fn handle_connection(&mut self) {
        self.send_line(220, String::from("Welcome! Patine build 0.1-dev"))
            .await;
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

    pub async fn send_line(&mut self, code: u16, message: String) {
        self.framed
            .send(SmtpResponse::SingleLine(code, message))
            .await
            .unwrap();
    }

    pub async fn send_multiline(&mut self, code: u16, message: Vec<String>) {
        self.framed
            .send(SmtpResponse::Multiline(code, message))
            .await
            .unwrap();
    }

    pub async fn starttls(&mut self) {
        match self.transaction_type {
            TransactionType::SERVER | TransactionType::SUBMISSION => {
                let old_framed = std::mem::replace(
                    &mut self.framed,
                    Framed::new(Box::new(tokio::io::empty()), SmtpCodec::new()),
                );

                let tcp_stream = old_framed.into_inner();
                let tls_stream = ACCEPTOR.accept(tcp_stream).await;

                if tls_stream.is_err() {
                    warn!("Invalid TLS data");
                    return;
                }

                self.framed = Framed::new(Box::new(tls_stream.unwrap()), SmtpCodec::new());
                self.tls = true;
                self.state = SmtpState::Connected;
                self.from = None;
                self.to = None;
            }
            _ => {
                panic!("Unsupported transaction type received")
            }
        }
    }
}

impl SmtpTransaction<SmtpResponse, SmtpCommand> {
    pub async fn new_client_from_submission(
        domain: String,
        from: String,
        to: String,
    ) -> Result<SmtpTransaction<SmtpResponse, SmtpCommand>, Error> {
        let mx_response = RESOLVER.mx_lookup(&domain).await?;
        let mut last_err = None;

        for mx in mx_response.iter().map(|mx| mx.exchange().to_utf8()) {
            let socket_addrs = format!("{}:25", mx)
                .to_socket_addrs()
                .map_err(|e| Error::from(e))?;

            for addr in socket_addrs {
                match TcpStream::connect(addr).await {
                    Ok(stream) => {
                        trace!("SmtpTransaction connected to {:?}", addr);
                        let mut client = Self::new_client(stream);
                        client.from = Some(from);
                        client.to = Some(vec![to]);
                        return Ok(client);
                    }
                    Err(e) => {
                        last_err = Some(Error::from(e));
                    }
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            Error::new(ErrorKind::ConnectionRefused, "No valid MX address found")
        }))
    }

    pub fn new_client(tcp_stream: TcpStream) -> SmtpTransaction<SmtpResponse, SmtpCommand> {
        Self {
            tls: false,
            esmtp: false,
            authenticated: false,
            state: SmtpState::Connected,
            from: None,
            to: None,
            transaction_type: TransactionType::CLIENT,
            framed: Framed::new(Box::new(tcp_stream), SmtpCodec::new()),
        }
    }

    // TODO: Add mail here as Vec<String>
    pub async fn handle_connection(&mut self, data: Vec<u8>) -> Result<(), Error> {
        self.expect_response(220).await?;

        self.framed.send(SmtpCommand::Ehlo(DOMAIN.get().unwrap().clone())).await?;
        let ehlo_response = self.expect_response(250).await?;
        if self.is_tls(ehlo_response) {
            self.framed.send(SmtpCommand::Starttls).await?;
            self.expect_response(220).await?;
        }

        self.framed
            .send(SmtpCommand::Mail(self.from.clone().unwrap()))
            .await?;
        self.expect_response(250).await?;

        if self.to.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No email provided"));
        }

        for recipient in self.to.clone().unwrap() {
            self.framed.send(SmtpCommand::Rcpt(recipient)).await?;
            self.expect_response(250).await?;
        }

        self.framed.send(SmtpCommand::Data).await?;
        self.expect_response(354).await?;

        self.framed.send(SmtpCommand::DataEnd(data)).await?;
        self.expect_response(250).await?;

        self.framed.send(Quit).await?;
        self.expect_response(250).await?;

        Ok(())
    }

    async fn expect_response(&mut self, expected_code: u16) -> Result<SmtpResponse, Error> {
        while let Some(result) = self.framed.next().await {
            return match result {
                Ok(SmtpResponse::SingleLine(code, msg)) => {
                    if code == expected_code {
                        Ok(SmtpResponse::SingleLine(code, msg))
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, msg))
                    }
                }
                Ok(SmtpResponse::Multiline(code, lines)) => {
                    if code == expected_code {
                        Ok(SmtpResponse::Multiline(code, lines))
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, lines.join("; ")))
                    }
                }
                Err(e) => Err(e),
            }
        }

        Err(Error::new(ErrorKind::UnexpectedEof, "Connection closed unexpectedly"))
    }

    fn is_tls(&self, smtp_response: SmtpResponse) -> bool {
        match smtp_response {
            SmtpResponse::SingleLine(_, _) => {
                false
            }
            SmtpResponse::Multiline(_, lines) => {
                if lines.contains(&"STARTTLS".to_string()) {
                    return true
                }
                false
            }
        }
    }

    fn starttls(&mut self) {

    }
}
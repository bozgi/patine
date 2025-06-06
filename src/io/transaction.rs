use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_server_codec::SmtpServerCodec;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use futures::{SinkExt, StreamExt};
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::Resolver;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::trace;
use crate::command::handlers::registry::HANDLERS;
use crate::io::smtp_codec::SmtpCodec;
use crate::io::tls::ACCEPTOR;
use crate::io::transaction_type::TransactionType;

trait AsyncIO: AsyncRead + AsyncWrite {}
impl<T: AsyncRead + AsyncWrite + ?Sized> AsyncIO for T {}

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
    pub fn new(tcp_stream: TcpStream) -> SmtpTransaction<SmtpCommand, SmtpResponse> {
        Self {
            tls: false,
            esmtp: false,
            authenticated: false,
            state: SmtpState::Connected,
            from: None,
            to: None,
            transaction_type: TransactionType::SERVER,
            framed: Framed::new(Box::new(tcp_stream), SmtpCodec::new()),
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

    pub async fn starttls(&mut self) {
        match self.transaction_type {
            TransactionType::SERVER | TransactionType::SUBMISSION => {
                let old_framed = std::mem::replace(
                    &mut self.framed,
                    Framed::new(Box::new(tokio::io::empty()), SmtpCodec::new()),
                );

                let tcp_stream = old_framed.into_inner();
                let tls_stream = ACCEPTOR.accept(tcp_stream).await.unwrap();

                self.framed = Framed::new(Box::new(tls_stream), SmtpServerCodec::new());
                self.tls = true;
                self.state = SmtpState::Connected;
                self.from = None;
                self.to = None;
            }
            TransactionType::CLIENT => {




                // resolver.mx_lookup()
            }
        }
    }
}

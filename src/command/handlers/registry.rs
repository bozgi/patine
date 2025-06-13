use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::command::command_handler::CommandHandler;
use crate::command::handlers::auth_handler::AuthHandler;
use crate::command::handlers::data_end_handler::DataEndHandler;
use crate::command::handlers::data_handler::DataHandler;
use crate::command::handlers::ehlo_handler::EhloHandler;
use crate::command::handlers::helo_handler::HeloHandler;
use crate::command::handlers::mail_handler::MailHandler;
use crate::command::handlers::noop_handler::NoopHandler;
use crate::command::handlers::quit_handler::QuitHandler;
use crate::command::handlers::rcpt_handler::RcptHandler;
use crate::command::handlers::rset_handler::RsetHandler;
use crate::command::handlers::starttls_handler::StarttlsHandler;
use crate::command::handlers::vrfy_handler::VrfyHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;

type ArcDynCommandHandler = Arc<dyn CommandHandler<In = SmtpCommand, Out = SmtpResponse> + Send + Sync>;

pub static HANDLERS: Lazy<HashMap<&'static str, ArcDynCommandHandler>> = Lazy::new(|| {
    let mut handlers: HashMap<&'static str, ArcDynCommandHandler> = HashMap::new();
    handlers.insert("ehlo", Arc::new(EhloHandler));
    handlers.insert("helo", Arc::new(HeloHandler));
    handlers.insert("data", Arc::new(DataHandler));
    handlers.insert("mail", Arc::new(MailHandler));
    handlers.insert("noop", Arc::new(NoopHandler));
    handlers.insert("quit", Arc::new(QuitHandler));
    handlers.insert("rcpt", Arc::new(RcptHandler));
    handlers.insert("rset", Arc::new(RsetHandler));
    handlers.insert("vrfy", Arc::new(VrfyHandler));
    handlers.insert("auth", Arc::new(AuthHandler));
    handlers.insert("data_end", Arc::new(DataEndHandler));
    handlers.insert("starttls", Arc::new(StarttlsHandler));
    handlers
});
use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::command::command_handler::CommandHandler;
use crate::command::handlers::data_end_handler::DataEndHandler;
use crate::command::handlers::data_handler::DataHandler;
use crate::command::handlers::ehlo_handler::EhloHandler;
use crate::command::handlers::helo_handler::HeloHandler;
use crate::command::handlers::mail_handler::MailHandler;
use crate::command::handlers::noop_handler::NoopHandler;
use crate::command::handlers::quit_handler::QuitHandler;
use crate::command::handlers::rcpt_handler::RcptHandler;
use crate::command::handlers::rset_handler::RsetHandler;
use crate::command::handlers::vrfy_handler::VrfyHandler;

pub static HANDLERS: Lazy<HashMap<&'static str, Arc<dyn CommandHandler + Send + Sync>>> = Lazy::new(|| {
    let mut handlers: HashMap<&'static str, Arc<dyn CommandHandler + Send + Sync>> = HashMap::new();
    handlers.insert("ehlo", Arc::new(EhloHandler));
    handlers.insert("helo", Arc::new(HeloHandler));
    handlers.insert("data", Arc::new(DataHandler));
    handlers.insert("mail", Arc::new(MailHandler));
    handlers.insert("noop", Arc::new(NoopHandler));
    handlers.insert("quit", Arc::new(QuitHandler));
    handlers.insert("rcpt", Arc::new(RcptHandler));
    handlers.insert("rset", Arc::new(RsetHandler));
    handlers.insert("vrfy", Arc::new(VrfyHandler));
    handlers.insert("data_end", Arc::new(DataEndHandler));
    handlers
});
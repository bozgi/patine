use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::transaction::SmtpTransaction;

pub struct StarttlsHandler;

#[async_trait]
impl CommandHandler for StarttlsHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Starttls = command {
            txn.send_line(220, String::from("TLS started")).await;
            txn.starttls().await;
        } else {
            txn.send_line(551, String::from("Unknown error")).await;
        }
    }
}
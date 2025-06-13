use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::transaction::SmtpTransaction;

pub struct NoopHandler;

#[async_trait]
impl CommandHandler for NoopHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Noop = command {
            txn.send_line(250, String::from("No operation performed")).await;
        } else {
            txn.send_line(551, String::from("Unknown error")).await;
        }
    }
}
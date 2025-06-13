use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::transaction::SmtpTransaction;

pub struct VrfyHandler;

#[async_trait]
impl CommandHandler for VrfyHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Vrfy(_) = command {
            txn.send_line(252, "Won't verify for security reasons".into()).await;
        }
    }
}
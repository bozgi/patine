use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

pub struct QuitHandler;

#[async_trait]
impl CommandHandler for QuitHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Quit = command {
            txn.state = SmtpState::Finished;
            txn.send_line(250, String::from("Goodbye")).await;
        } else {
            txn.send_line(554, String::from("Unknown error")).await;
        }
    }
}
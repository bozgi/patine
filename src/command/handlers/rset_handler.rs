use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

pub struct RsetHandler;

#[async_trait]
impl CommandHandler for RsetHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        if let SmtpCommand::Rset = command {
            txn.state = SmtpState::Connected;
            txn.esmtp = false;
            txn.tls = false;
            txn.from = None;
            txn.to = None;
            txn.send_line(250, String::from("Okie")).await;
        } else {
            txn.send_line(554, String::from("Unknown error")).await;
        }
    }
}
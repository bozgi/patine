use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

pub struct HeloHandler;

#[async_trait]
impl CommandHandler for HeloHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        if let SmtpCommand::Helo(domain) = command {
            if domain.trim().is_empty() {
                txn.send_line(501, String::from("Invalid argument")).await;
            }

            match txn.state {
                SmtpState::Connected => {
                    txn.state = SmtpState::Greeted;
                    txn.esmtp = false;
                    txn.send_line(250, String::from("Hello!")).await;
                }
                _ => txn.send_line(503, String::from("Bad sequence of commands")).await
            }
        } else {
            txn.send_line(554, String::from("Unknown error")).await;
        }
    }
}
use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

pub struct EhloHandler;

#[async_trait]
impl CommandHandler for EhloHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        if let SmtpCommand::Ehlo(domain) = command {
            if domain.trim().is_empty() {
                txn.send_line(501, String::from("Invalid argument")).await;
                return;
            }

            match txn.state {
                SmtpState::Connected => {
                    txn.state = SmtpState::Greeted;
                    txn.esmtp = true;
                    txn.send_line(250, String::from("Hello!")).await;
                }
                _ => txn.send_line(503, String::from("Bad sequence of commands")).await
            }
        } else {
            txn.send_line(554, String::from("Unknown error")).await;
        }
    }
}
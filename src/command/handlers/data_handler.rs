use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

pub struct DataHandler;

#[async_trait]
impl CommandHandler for DataHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        if let SmtpCommand::Data = command {
            match txn.state {
                SmtpState::Addressing => {
                    txn.state = SmtpState::Sending;
                    txn.send_line(354, String::from("Alright, go on")).await
                }
                _ => txn.send_line(503, String::from("Bad sequence of commands")).await
            }
        } else {
            txn.send_line(551, String::from("Unknown error")).await;
        }
    }
}
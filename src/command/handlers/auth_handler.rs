use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::handlers::data_handler::DataHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

struct AuthHandler;

#[async_trait]
impl CommandHandler for AuthHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        if let SmtpCommand::Auth(data) = command {
            if !txn.tls {
                txn.send_line(530, "Must issue a STARTTLS command first".to_string()).await;
                return;
            }
            match txn.state {
                SmtpState::Greeted => {
                    todo!()
                }
                _ => {}
            }
        } else {
            txn.send_line(551, String::from("Unknown error")).await;
        }
    }
}
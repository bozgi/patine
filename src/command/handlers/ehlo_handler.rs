use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::io::transaction_type::TransactionType;

pub struct EhloHandler;

#[async_trait]
impl CommandHandler for EhloHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Ehlo(domain) = command {
            if domain.trim().is_empty() {
                txn.send_line(501, String::from("Invalid argument")).await;
                return;
            }

            match txn.state {
                SmtpState::Connected => {
                    txn.state = SmtpState::Greeted;
                    txn.esmtp = true;
                    
                    let mut response = Vec::with_capacity(7);
                    response.push("Welcome!".to_string());
                    response.push("SIZE 35882577".to_string());
                    response.push("8BITMIME".to_string());
                    response.push("SMTPUTF8".to_string());
                    
                    if txn.tls {
                        response.push("AUTH PLAIN".to_string());
                    } else {
                        response.push("STARTTLS".to_string());
                    }

                    txn.send_multiline(250, response).await;
                }
                _ => txn.send_line(503, String::from("Bad sequence of commands")).await
            }
        } else {
            txn.send_line(554, String::from("Unknown error")).await;
        }
    }
}
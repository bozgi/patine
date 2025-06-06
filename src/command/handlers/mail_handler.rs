use async_trait::async_trait;
use regex::Regex;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;

pub struct MailHandler;

#[async_trait]
impl CommandHandler for MailHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Mail(arg) = command {
            let arg = arg.trim();

            if !arg.to_uppercase().starts_with("FROM:") {
                txn.send_line(501, "Syntax error in parameters or arguments".into()).await;
                return;
            }

            let address = arg[5..].trim();
            let address = address.strip_prefix('<').and_then(|s| s.strip_suffix('>'));

            let re = Regex::new(r"^[\w\-.]+@([\w\-]+\.)+[\w\-]{2,}$").unwrap();

            if let Some(address) = address {
                if !re.is_match(address) {
                    txn.send_line(501, "Invalid email format".into()).await;
                    return;
                }

                match txn.state {
                    SmtpState::Greeted => {
                        if !txn.authenticated {
                            txn.send_line(530, "Authentication Required.".into()).await;
                            return;
                        }
                        
                        txn.state = SmtpState::Mailing;
                        txn.to = Some(Vec::with_capacity(1));
                        txn.from = Some(address.to_string());
                        txn.send_line(250, "Sender OK".into()).await;
                    }
                    _ => {
                        txn.send_line(503, "Bad sequence of commands".into()).await;
                    }
                }
            } else {
                txn.send_line(501, "Parameter syntax error".into()).await;
            }
        } else {
            txn.send_line(500, "Unknown error".into()).await;
        }
    }
}

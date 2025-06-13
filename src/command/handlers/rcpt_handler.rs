use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::io::transaction_type::TransactionType;
use crate::storage::maildir::DOMAIN;
use async_trait::async_trait;
use regex::Regex;
use tracing::trace;

pub struct RcptHandler;

#[async_trait]
impl CommandHandler for RcptHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Rcpt(arg) = command {
            let arg = arg.trim();

            if !arg.to_uppercase().starts_with("TO:") {
                txn.send_line(501, "Syntax error in parameters or arguments".into())
                    .await;
                return;
            }

            let address = arg[3..].trim();
            let address = address.strip_prefix('<').and_then(|s| s.strip_suffix('>'));

            let re = Regex::new(r"^[\w\-.]+@([\w\-]+\.)+[\w\-]{2,}$").unwrap();

            if let Some(address) = address {
                if !re.is_match(address) {
                    txn.send_line(501, "Invalid email format".into()).await;
                    return;
                }

                match txn.state {
                    SmtpState::Mailing | SmtpState::Addressing => {
                        txn.state = SmtpState::Addressing;
                        let domain = address[address.chars().position(|c| c == '@').unwrap() + 1..].trim();
                        trace!("Domain: {}", domain);
                        if txn.transaction_type == TransactionType::SERVER && domain != DOMAIN.get().unwrap() {
                            txn.send_line(550, "Cannot relay".to_string()).await;
                            return;
                        }

                        txn.to
                            .as_mut()
                            .expect("State guarantees the existence of Vec here")
                            .push(address.to_string());
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

use std::fmt::format;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::storage::maildir::{DOMAIN, check_maildir, write_to_maildir};
use async_trait::async_trait;
use tracing::{error, warn};

pub struct DataEndHandler;

#[async_trait]
impl CommandHandler for DataEndHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::DataEnd(body) = command {
            tracing::info!("Received email");

            let mailboxes = txn.to.take().unwrap();

            for mailbox in mailboxes {
                if let Some((mailbox, domain)) = mailbox.split_once("@") {
                    if domain == DOMAIN.get().expect("DOMAIN set in main") {
                        check_maildir(mailbox).await.unwrap();
                        write_to_maildir(mailbox, &body).await.unwrap();
                    } else {
                        let transaction = SmtpTransaction::new_client_from_submission(
                            domain.to_string(),
                            txn.from.clone().unwrap(),
                            format!("{}@{}", mailbox, domain)
                        )
                        .await;
                        if let Err(e) = transaction {
                            warn!("Failed to create SMTP transaction: {}", e);
                            continue;
                        }

                        let mut transaction = transaction.unwrap();
                        let cloned = body.clone();
                        tokio::spawn(async move {
                            transaction.handle_connection(cloned).await.unwrap();
                        });
                    }
                }
            }

            txn.state = SmtpState::Greeted;
            txn.from = None;
            txn.to = None;

            txn.send_line(250, "OK: queued".to_string()).await;
        }
    }
}

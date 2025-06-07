use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::storage::maildir::{DOMAIN, check_maildir, write_to_maildir};
use async_trait::async_trait;
use std::fmt::format;
use tokio::spawn;
use tokio::task::spawn_blocking;
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
                    if domain == DOMAIN.get().unwrap() {
                        check_maildir(mailbox).await.unwrap();
                        write_to_maildir(mailbox, &body).await.unwrap();
                    } else {
                        let from = txn.from.clone().unwrap();
                        let to = format!("{}@{}", mailbox, domain);
                        let body_clone = body.clone();

                        spawn(async move {
                            let transaction = SmtpTransaction::new_client_from_submission(
                                domain.to_string(),
                                from,
                                to,
                            )
                            .await;

                            match transaction {
                                Ok(mut transaction) => {
                                    if let Err(e) = transaction.handle_connection(body_clone).await
                                    {
                                        warn!("Relay error: {}", e);
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to create SMTP transaction: {}", e);
                                }
                            }
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

use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::storage::maildir::{check_maildir, write_to_maildir, DOMAIN};
use async_trait::async_trait;
use tokio::spawn;
use tracing::warn;

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
                if let Some((user, domain)) = mailbox.split_once("@") {
                    let user = user.to_owned();
                    let domain = domain.to_owned();
                    if &domain == DOMAIN.get().unwrap() {
                        check_maildir(&user).await.unwrap();
                        write_to_maildir(&user, &body).await.unwrap();
                    } else {
                        let from = txn.from.clone().unwrap();
                        let to = format!("{}@{}", user, domain);
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
                            };
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

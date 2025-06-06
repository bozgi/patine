use std::path::{Path, PathBuf};
use std::ptr::write;
use async_trait::async_trait;
use tracing_subscriber::fmt::format::Format;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_response::SmtpResponse;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::storage::maildir::{check_maildir, write_to_maildir, DOMAIN, MAILDIR_ROOT};

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
                        unimplemented!("Forwarding is not implemented yet!");
                    }

                    txn.state = SmtpState::Greeted;
                    txn.from = None;
                    txn.to = None;
                }
            }

            txn.send_line(250, "OK: queued".to_string()).await;
        }
    }
}
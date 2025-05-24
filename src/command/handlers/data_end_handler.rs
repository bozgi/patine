use std::path::{Path, PathBuf};
use std::ptr::write;
use async_trait::async_trait;
use tracing_subscriber::fmt::format::Format;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use crate::storage::maildir::{check_maildir, write_to_maildir, MAILDIR_ROOT};

pub struct DataEndHandler;

#[async_trait]
impl CommandHandler for DataEndHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        if let SmtpCommand::DataEnd(body) = command {
            tracing::info!("Received email");

            let mailboxes = txn.to.take().unwrap();

            for mailbox in mailboxes {
                if let Some((mailbox, domain)) = mailbox.split_once("@") {
                    if domain == "bozgi.space" {
                        check_maildir(mailbox).await.unwrap();
                        write_to_maildir(Path::new(&mailbox), &body).await.unwrap();
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
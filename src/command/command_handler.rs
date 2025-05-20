use async_trait::async_trait;
use crate::command::smtp_command::SmtpCommand;
use crate::io::transaction::SmtpTransaction;

#[async_trait]
pub trait CommandHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand);
}
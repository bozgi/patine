use async_trait::async_trait;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::transaction::SmtpTransaction;

pub struct MailHandler;

#[async_trait]
impl CommandHandler for MailHandler {
    async fn handle(&self, txn: &mut SmtpTransaction, command: SmtpCommand) {
        todo!()
    }
}
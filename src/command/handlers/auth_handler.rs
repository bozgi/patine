use std::process::Stdio;
use crate::command::command_handler::CommandHandler;
use crate::command::smtp_command::SmtpCommand;
use crate::io::smtp_state::SmtpState;
use crate::io::transaction::SmtpTransaction;
use async_trait::async_trait;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tracing::{error, trace};
use crate::io::smtp_response::SmtpResponse;
use crate::storage::maildir::PAM_HELPER_PATH;

pub struct AuthHandler;

#[async_trait]
impl CommandHandler for AuthHandler {
    type In = SmtpCommand;
    type Out = SmtpResponse;

    async fn handle(&self, txn: &mut SmtpTransaction<Self::In, Self::Out>, command: SmtpCommand) {
        if let SmtpCommand::Auth(data) = command {
            if !txn.tls {
                txn.send_line(530, "Must issue a STARTTLS command first".to_string()).await;
                return;
            }
            match txn.state {
                SmtpState::Greeted => {
                    trace!("{}", data);
                    let data = data.strip_prefix("PLAIN");
                    if data.is_none() {
                        txn.send_line(501, String::from("Invalid argument")).await;
                        return;
                    }
                    let data = data.unwrap().trim();
                    let decoded = BASE64_STANDARD.decode(data);
                    if decoded.is_err() {
                        txn.send_line(501, String::from("Invalid argument")).await;
                        return;
                    }
                    let decoded = decoded.unwrap();

                    let parts: Vec<&[u8]> = decoded.split(|&b| b == 0).skip(1).collect();

                    let username = String::from_utf8(parts[0].to_vec()).unwrap();
                    let password = String::from_utf8(parts[1].to_vec()).unwrap();

                    if authenticate_user(username, password).await {
                        txn.authenticated = true;
                        txn.send_line(235, "Authentication succeeded".into()).await;
                    } else {
                        txn.send_line(535, "Authentication credentials invalid".into()).await;
                    }


                }
                _ => {}
            }
        } else {
            txn.send_line(551, String::from("Unknown error")).await;
        }
    }
}

pub async fn authenticate_user(username: String, password: String) -> bool {
    trace!("{} {}", username, password);

    let mut child = Command::new("sudo")
        .arg("-n")
        .arg(format!("{}/pam_helper", PAM_HELPER_PATH.get().unwrap()))
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start pam_helper");

    if let Some(mut stdin) = child.stdin.take() {
        if stdin.write_all(format!("{}\n{}\n", username, password).as_bytes()).await.is_err() {
            error!("Failed to write to pam_helper stdin");
            return false;
        }
    }

    match child.wait().await {
        Ok(status) => status.success(),
        Err(e) => {
            error!("Could not authenticate user, error {}", e);
            false
        },
    }
}
mod command;
mod io;
mod storage;

use crate::storage::maildir::{DOMAIN, MAILDIR_ROOT, PAM_HELPER_PATH};
use io::transaction::SmtpTransaction;
use std::sync::OnceLock;
use std::{env, process};
use tokio::net::TcpListener;
use tracing::{Level, error, info, debug};

static SUBMISSION_PORT: OnceLock<u16> = OnceLock::new();
static RELAY_PORT: OnceLock<u16> = OnceLock::new();

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Patine prototype initializing...");

    load_config();

    let submission_listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        SUBMISSION_PORT.get().expect("Value set below")
    ))
    .await
    .unwrap();
    let relay_listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        RELAY_PORT.get().expect("Value set below")
    ))
    .await
    .unwrap();

    info!(
        "Patine prototype running ({}/submission + {}/relay)",
        SUBMISSION_PORT.get().unwrap(),
        RELAY_PORT.get().unwrap()
    );

    let submission_task = tokio::spawn(async move {
        loop {
            let (socket, addr) = submission_listener.accept().await.unwrap();
            tokio::spawn(async move {
                info!("[SUBMISSION] Accepted connection from {:?}", addr);
                let mut transaction = SmtpTransaction::new_submission(socket);
                transaction.handle_connection().await;
            });
        }
    });

    let relay_task = tokio::spawn(async move {
        loop {
            let (socket, addr) = relay_listener.accept().await.unwrap();
            tokio::spawn(async move {
                info!("[RELAY] Accepted connection from {:?}", addr);
                let mut transaction = SmtpTransaction::new_server(socket);
                transaction.handle_connection().await;
            });
        }
    });

    let _ = tokio::join!(submission_task, relay_task);
}

fn load_config() {
    dotenv::dotenv().ok();

    for (key, value) in env::vars() {
        if key == "MAILDIR_ROOT" {
            MAILDIR_ROOT.set(value).expect("MAILDIR set only here");
        } else if key == "DOMAIN" {
            DOMAIN.set(value).expect("DOMAIN set only here");
        } else if key == "RELAY_PORT" {
            RELAY_PORT
                .set(value.parse::<u16>().expect("RELAY_PORT should be a number"))
                .expect("RELAY_PORT set only here");
        } else if key == "SUBMISSION_PORT" {
            SUBMISSION_PORT
                .set(
                    value
                        .parse::<u16>()
                        .expect("SUBMISSION_PORT should be a number"),
                )
                .expect("SUBMISSION_PORT set only here");
        } else if key == "PAM_HELPER_PATH" {
            PAM_HELPER_PATH
                .set(value)
                .expect("PAM_HELPER_PATH set only`")
        }
    }

    let mut error_flag = false;

    if MAILDIR_ROOT.get().is_none() {
        error!("MAILDIR_ROOT not set");
        error_flag = true;
    }

    if DOMAIN.get().is_none() {
        error!("DOMAIN not set");
        error_flag = true;
    }

    if SUBMISSION_PORT.get().is_none() {
        error!("SUBMISSION_PORT not set");
        error_flag = true;
    }

    if RELAY_PORT.get().is_none() {
        error!("RELAY_PORT not set");
        error_flag = true;
    }

    if PAM_HELPER_PATH.get().is_none() {
        error!("PAM_HELPER_PATH not set");
        error_flag = true;
    }

    if error_flag {
        error!("Errors occured while loading config file, exiting...");
        process::exit(1);
    }

    info!("Loaded config file");
    debug!("MAILDIR_ROOT={:#?}", MAILDIR_ROOT.get().unwrap());
    debug!("DOMAIN={:#?}", DOMAIN.get().unwrap());
    debug!("SUBMISSION_PORT={:#?}", SUBMISSION_PORT.get().unwrap());
    debug!("RELAY_PORT={:#?}", RELAY_PORT.get().unwrap());
    debug!("PAM_HELPER_PATH={:#?}", PAM_HELPER_PATH.get().unwrap());
}

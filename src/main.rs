mod command;
mod io;
mod storage;

use std::{env, process};
use std::path::Path;
use tokio::net::TcpListener;
use tracing::{error, info, trace, Level};
use io::transaction::SmtpTransaction;
use crate::storage::maildir::{DOMAIN, MAILDIR_ROOT};

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Patine prototype initializing...");

    load_config();

    if !Path::new(MAILDIR_ROOT.get().expect("Already set")).exists() {
        error!("Root for mail storage does not exist! Aborting...");
        return;
    }

    let listener = TcpListener::bind("127.0.0.1:4450").await.unwrap();

    info!("Patine prototype running on port 4450");

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            info!("Accepted connection from {:?}", addr);
            let mut transaction = SmtpTransaction::new_server(socket);
            transaction.handle_connection().await;
        });
    }
}

fn load_config() {
    dotenv::dotenv().ok();

    for (key, value) in env::vars() {
        if key == "MAILDIR" {
            MAILDIR_ROOT.set(value).expect("MAILDIR set only here");
        } else if key == "DOMAIN" {
            DOMAIN.set(value).expect("DOMAIN set here");
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

    if error_flag {
        error!("Errors occured while loading config file, exiting...");
        process::exit(1);
    }
}
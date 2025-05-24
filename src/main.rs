mod command;
mod server;
mod io;
mod storage;

use std::path::Path;
use tokio::net::TcpListener;
use tracing::{error, info, Level};
use io::transaction::SmtpTransaction;
use crate::storage::maildir::MAILDIR_ROOT;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Patine prototype initializing...");

    if !Path::new(MAILDIR_ROOT).exists() {
        error!("Root for mail storage does not exist! Aborting...");
        return;
    }

    let listener = TcpListener::bind("127.0.0.1:4450").await.unwrap();

    info!("Patine prototype running on port 4450");

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            info!("Accepted connection from {:?}", addr);
            let mut transaction = SmtpTransaction::new(socket);
            transaction.handle_connection().await;
        });
    }
}

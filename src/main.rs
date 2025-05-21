mod command;
mod server;
mod io;
mod storage;

use tokio::net::TcpListener;
use tracing::Level;
use io::transaction::SmtpTransaction;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let listener = TcpListener::bind("127.0.0.1:4450").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            tracing::info!("Accepted connection from {:?}", addr);
            let mut transaction = SmtpTransaction::new(socket);
            transaction.handle_connection().await;
        });
    }
}

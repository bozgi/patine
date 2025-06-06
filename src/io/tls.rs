use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::rustls::{ClientConfig, RootCertStore, ServerConfig};
use tokio_rustls::{TlsAcceptor, TlsConnector};

pub static ACCEPTOR: Lazy<TlsAcceptor> = Lazy::new(|| {
    let certs = CertificateDer::pem_file_iter("/etc/mailcerts/cert.pem")
        .expect("Failed to load certs")
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to load certs");
    let key = PrivateKeyDer::from_pem_file("/etc/mailcerts/cert.key.pem");

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key.unwrap()).unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));

    acceptor
});

pub static CONNECTOR: Lazy<TlsConnector> = Lazy::new(|| {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    
    connector
});
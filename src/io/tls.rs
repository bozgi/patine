use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

pub static ACCEPTOR: Lazy<TlsAcceptor> = Lazy::new(|| {
    let certs = CertificateDer::from_pem_file("certs/cert.pem");
    let key = PrivateKeyDer::from_pem_file("certs/cert.key.pem");

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certs.unwrap()], key.unwrap()).unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));

    acceptor
});
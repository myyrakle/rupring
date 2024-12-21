use std::{fs, io, sync::Arc};

use rustls::ServerConfig;
use tokio_rustls::{
    rustls::pki_types::{CertificateDer, PrivateKeyDer},
    TlsAcceptor,
};

use crate::application_properties::ApplicationProperties;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    rustls_pemfile::certs(&mut reader).collect()
}

// Load private key from file.
fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}

pub fn new_tls_acceptor(
    application_properties: &ApplicationProperties,
) -> anyhow::Result<TlsAcceptor> {
    let certs = load_certs(&application_properties.server.ssl.cert)?;
    // Load private key.
    let key = load_private_key(&application_properties.server.ssl.key)?;

    let mut server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| error(e.to_string()))?;

    if application_properties.server.http2.enabled {
        server_config.alpn_protocols =
            vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
    } else {
        server_config.alpn_protocols = vec![b"http/1.1".to_vec(), b"http/1.0".to_vec()];
    }

    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));

    Ok(tls_acceptor)
}

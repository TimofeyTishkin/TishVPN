use std::{fs::File, io::BufReader};

use anyhow::Result;
use rustls::RootCertStore;
use rustls::pki_types::CertificateDer;
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn load_certs(path: &str) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    let mut reader = BufReader::new(File::open(path)?);
    Ok(certs(&mut reader).collect::<Result<_, _>>()?)
}

pub fn load_key(path: &str) -> Result<rustls::pki_types::PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(File::open(path)?);
    let keys = pkcs8_private_keys(&mut reader).collect::<Result<Vec<_>, _>>()?;
    Ok(keys[0].clone_key().into())
}

pub fn load_ca(path: &str) -> Result<RootCertStore> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut store = RootCertStore::empty();

    for cert in certs(&mut reader) {
        store.add(cert?)?;
    }

    Ok(store)
}

pub fn extract_san(cert: &CertificateDer) -> Vec<String> {
    use x509_parser::prelude::*;

    let (_, parsed) = X509Certificate::from_der(cert.as_ref()).unwrap();

    parsed
        .extensions()
        .iter()
        .filter_map(|ext| {
            if let ParsedExtension::SubjectAlternativeName(san) = ext.parsed_extension() {
                Some(
                    san.general_names
                        .iter()
                        .filter_map(|name| match name {
                            GeneralName::DNSName(dns) => Some(dns.to_string()),
                            GeneralName::URI(uri) => Some(uri.to_string()),
                            GeneralName::IPAddress(ip) => Some(format!("{:?}", ip)),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

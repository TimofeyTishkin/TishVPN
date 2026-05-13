// Server
// Create tun0
// Create TCP socket
// Listen for incoming TLS sessions
// Check the cert
/* If cert is correct
-provide vpn on proxy traffic (depending on given cert)
transmit raw data via encrypted tls session
if not -> show html_decoy
*/

/*use std::{fs::File, io::BufReader, sync::Arc};
use x509_parser::extensions::GeneralName;
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls::{
    server::{WebPkiClientVerifier, ServerConfig},
    RootCertStore,
};
*/

use std::{net::Ipv4Addr, sync::Arc};
use anyhow::Result;
use rustls::server::{WebPkiClientVerifier, ServerConfig};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tokio_tun::Tun;
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use rustls::crypto::ring::default_provider;
mod certs;
use certs::{load_certs, load_key, load_ca, extract_san};

const MTU: usize = 1500;

#[tokio::main]
async fn main() -> Result<()> {
    default_provider().install_default().expect("crypto provider");
    let certs = load_certs("pki/server.crt")?;
    let key = load_key("pki/server.key")?;
    let client_ca = load_ca("pki/ca.crt")?;

    let verifier = WebPkiClientVerifier::builder(client_ca.into()).build()?;

    let config = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(certs, key)?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind("0.0.0.0:443").await?;
    println!("Server listening on 443");

    loop {
	let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
	let tun = Tun::builder()
		.name("tun0")
		.address(Ipv4Addr::new(10,0,0,1))
		.netmask(Ipv4Addr::new(255,255,255,0))
		.up()
		.build()
		.expect("Error while creating tun interface")
		.pop()
		.unwrap();

        tokio::spawn(async move {
            println!("---Started new client handler---");
            if let Err(e) = handle_client(tun, stream, acceptor).await {
                eprintln!("error: {:?}", e);
            }
        });
    }
}

async fn handle_client(tun: Tun, stream: TcpStream, acceptor: TlsAcceptor) -> Result<()> {
    let tls = acceptor.accept(stream).await?;

    let certs = tls.get_ref().1.peer_certificates()
        .ok_or_else(|| anyhow::anyhow!("No client cert"))?; println!("---Getting peer certificates---");

    let client_cert = &certs[0];

    let san_contains_vpn = extract_san(client_cert)
        .iter()
        .any(|s| s.contains("vpn"));

    if san_contains_vpn {
        let _ = aaa(tun, tls).await;
    } else {
        let _ = aaa(tun, tls).await;
        /*let mut file = tokio::fs::File::open("index.html").await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;
        println!("Read index.html successfully. Transmitting...");
        tls.write_all(&content).await?;*/
    }

    Ok(())
}

async fn aaa(tun: Tun, tls: TlsStream<TcpStream>) -> anyhow::Result<()> {
    // TODO: логика
    println!("AAA triggered");
    //let peer: std::net::SocketAddr = peer_addr.parse()?;
    let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);
    let (mut tls_reader, mut tls_writer) = tokio::io::split(tls);
    //let udp = std::sync::Arc::new(udp);
    //let udp_recv = udp.clone();
	
    let send_task = tokio::spawn(async move {
	let mut buf = vec![0u8; MTU];
	loop{
		let n = tun_reader.read(&mut buf).await?;
		tls_writer.write_all(&buf[..n]).await?;
		//udp.send_to(&buf[..n], peer).await?;
	}
	#[allow(unreachable_code)]
	anyhow::Ok(())
    });

    let recv_task = tokio::spawn(async move {
	let mut buf = vec![0u8; MTU];
	loop{
		//let (n, _src) = udp_recv.recv_from(&mut buf).await?;
		let n = tls_reader.read(&mut buf).await?;
		tun_writer.write_all(&buf[..n]).await?;
	}
	#[allow(unreachable_code)]
	anyhow::Ok(())
    });
	
    tokio::try_join!(
	async {send_task.await?},
	async {recv_task.await?}
    )?;
    Ok(())
}

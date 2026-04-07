//use std::sync::Arc;
//use tokio_tun::Tun;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
mod tun;
mod tunnel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let tun = Tun::builder().name("tun0").address(Ipv4Addr::new(10,0,0,1)).netmask(Ipv4Addr::new(255,255,255,0)).up().build().expect("Interface tun0 is up").pop().unwrap();
    //let tun  = Arc::new(tun::tun("tun0", Ipv4Addr::new(10,0,0,1), Ipv4Addr::new(255,255,255,0)).await.expect("Error creating tun0"));
    //let tun_tx = tun.clone();
    
    //tokio::spawn(async move {
    //	let dummy_data = b"hello from rust";
	//if let Err(e) = tun_tx.send_all(dummy_data).await {
	  //  eprintln!("Error while sending {}", e);
	//}
    //});
    
    //let mut buf = [0u8; 1500];
    //loop {
    //    let n = tun.recv(&mut buf).await?;    
//	println!("Received {} b from Tun: {:?}", n, &buf[..n]);
  //  }


    //creating tun interface
    let tun = tun::tun("tun0", Ipv4Addr::new(10,0,0,1), Ipv4Addr::new(255,255,255,0)).await.expect("Error creating tun0");
	
    let udp = UdpSocket::bind("0.0.0.0:51120").await.unwrap();
	
    //tunnel::run_tunnel(tun, udp, "192.168.100.61:51120").await?;
    tunnel::run_tunnel(tun, udp, "192.168.100.61:51120").await?;
    Ok(())
}
/*
fn main() {
    println!("Hello, world!");
}*/

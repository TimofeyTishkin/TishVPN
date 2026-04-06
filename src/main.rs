use std::sync::Arc;
//use tokio_tun::Tun;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::Ipv4Addr;
mod tun;

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
    let _tun = Arc::new(tun::tun("tun0", Ipv4Addr::new(10,0,0,1), Ipv4Addr::new(255,255,255,0)).await.expect("Error creating tun0"));
    loop{}
}
/*
fn main() {
    println!("Hello, world!");
}*/

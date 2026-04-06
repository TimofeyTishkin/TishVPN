//use std::sync::Arc;
use tokio_tun::Tun;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::Ipv4Addr;

//#[tokio::main]
pub async fn tun(name: &str, ipv4: Ipv4Addr, mask: Ipv4Addr) -> Result<Tun, Box<dyn std::error::Error>> {
    let tun = Tun::builder()
		.name(name)
		.address(ipv4)
		.netmask(mask)
		.up()
		.build()?
		.pop()
		.ok_or(format!("Error while creating {} interface", name))?;
		//.expect("Interface tun0 is up")
		//.pop()
		//.unwrap();
    Ok(tun)
    
}

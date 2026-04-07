use tokio::net::UdpSocket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tun::Tun;

const MTU: usize = 1500;

pub async fn run_tunnel(tun: Tun, udp: UdpSocket, peer_addr: &str) -> anyhow::Result<()>{
	let peer: std::net::SocketAddr = peer_addr.parse()?;
	let (mut tun_reader, mut tun_writer) = tokio::io::split(tun);
	let udp = std::sync::Arc::new(udp);
	let udp_recv = udp.clone();
	
	let send_task = tokio::spawn(async move {
		let mut buf = vec![0u8; MTU];
		loop{
			let n = tun_reader.read(&mut buf).await?;
			udp.send_to(&buf[..n], peer).await?;
		}
		#[allow(unreachable_code)]
		anyhow::Ok(())
	});

	let recv_task = tokio::spawn(async move {
		let mut buf = vec![0u8; MTU];
		loop{
			let (n, _src) = udp_recv.recv_from(&mut buf).await?;
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

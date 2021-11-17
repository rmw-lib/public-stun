use async_std::net::UdpSocket;
use rmw_stun::external_addr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

async fn _addr(server: &str) -> Option<SocketAddr> {
  let udp = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
    .await
    .unwrap();
  let start = Instant::now();
  let addr = external_addr(&udp, server, 3).await;
  let duration = start.elapsed();
  println!("{} : {:?} cost {:?}", server, addr, duration);
  addr
}

#[async_std::main]
async fn main() {
  let server = "stun.cablenet-as.net:3478";
  _addr(server).await;
  //let timeout = 3;
  //let addr = ;
}

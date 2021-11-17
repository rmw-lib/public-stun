use anyhow::Result;
use async_std::net::UdpSocket;
use async_std::task::block_on;
use rayon::prelude::*;
use rmw_stun::external_addr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

async fn _addr(server: String) -> (String, Option<SocketAddr>, Duration) {
  let udp = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
    .await
    .unwrap();
  let start = Instant::now();
  let addr = block_on(external_addr(&udp, &server, 3));
  let duration = start.elapsed();
  println!("{} : {:?} cost {:?}", &server, addr, duration);
  (server, addr, duration)
}

#[async_std::main]
async fn main() -> Result<()> {
  rayon::ThreadPoolBuilder::new()
    .num_threads(64)
    .build_global()?;

  let mut dir = std::env::current_exe()?;
  dir.pop();
  dir.pop();
  dir.pop();

  let mut ing = Vec::new();

  let file = File::open(dir.clone().join("stun.txt"))?;
  let reader = BufReader::new(file);
  for line in reader.lines() {
    let line = line?.trim().to_owned();
    if line.starts_with("#") || line.chars().count() == 0 {
      continue;
    }
    ing.push(line);
  }

  let mut li = Vec::<(String, Duration)>::new();

  for i in ing
    .par_iter()
    .map(|host| block_on(_addr(host.to_string())))
    .collect::<Vec<_>>()
  {
    if let (server, Some(_), duration) = i {
      li.push((server, duration));
    }
  }

  li.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

  let mut file = File::create(dir.join("li.txt"))?;
  file.write_all(
    li.iter()
      .map(|x| &x.0[..])
      .collect::<Vec<_>>()
      .join("\n")
      .as_bytes(),
  )?;

  //let timeout = 3;
  //let addr = ;
  Ok(())
}

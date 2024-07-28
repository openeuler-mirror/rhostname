use clap::Parser;
use std::{fs, net::IpAddr, net::SocketAddr, process};
use uthostname::{gethostname,getdomainname,sethostname};
use dns_lookup::{getnameinfo, AddrInfoHints, getaddrinfo};
use local_ip_address::list_afinet_netifas;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(
    short = 'a',
    long = "alias",
    help = "alias names",
  )]
  alias: bool,

  #[arg(
    short = 'A',
    long = "all-fqdns",
    help = "all long host names (FQDNs)",
  )]
  all_fqdns: bool,

  #[arg(
    short = 'b',
    long = "bool",
    help = "set default hostname if none available",
  )]
  boot: Option<String>,
  
  #[arg(
    short = 'd',
    long = "domain",
    help = "DNS domain name",
  )]
  domain: bool,

  #[arg(
    short = 'f',
    long = "fqdn",
    alias = "long",
    help = "long host name (FQDN)",
  )]
  fqdn: bool,

  #[arg(
    short = 'F',
    long = "file",
    help = "read host name or NIS domain name from given file",
  )]
  file: Option<String>,

  #[arg(
    short = 'i',
    long = "ip-address",
    help = "addresses for the host name",
  )]
  ip_address: bool,

  #[arg(
    short = 'I',
    long = "all-ip-address",
    help = "all addresses for the host",
  )]
  all_ip_address: bool,

  #[arg(
    short = 's',
    long = "short",
    help = "short host name",
  )]
  short: bool,

  #[arg(
    short = 'y',
    long = "yp",
    alias = "nis",
    help = "NIS/YP domain name",
  )]
  nis: bool,

  hostname: Option<String>,
}

fn run(args: Args) -> Result<(), &'static str> {
  if let Some(hostname) = args.boot {
    sethostname(hostname)
  } else if let Some(path) = args.file {
    let contents =
      fs::read_to_string(path)
      .expect("File not exist.");
    let mut hostname = "";

    for line in contents.lines() {
      if line != "" && !line.starts_with("#") {
        hostname = line;
        break;
      }
    }

    sethostname(hostname.to_string())
  } else if let Some(hostname) = args.hostname {
    sethostname(hostname)
  } else if args.alias {
    Ok(())
  } else if args.all_fqdns || args.all_ip_address {
    let network_interfaces = list_afinet_netifas();

    if let Ok(network_interfaces) = network_interfaces {
      for (_, ip) in network_interfaces.iter() {
        if !ip.is_ipv4() && !ip.is_ipv6()
          || ip.is_loopback()
          || ip.to_string().is_empty()
        {
          continue;
        }

        if let IpAddr::V6(ipv6) = ip {
          // About link local 
          // https://support.huawei.com/enterprise/zh/doc/EDOC1100116138
          // To avoid using rust nightly, source code is copied here
          // https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html#method.is_unicast_link_local
          // https://doc.rust-lang.org/1.80.0/src/core/net/ip_addr.rs.html#1626
          if (ipv6.segments()[0] & 0xffc0) == 0xfe80 {
            continue;
          }
        }

        let flags = if args.all_fqdns {0} else {1};

        let socket = SocketAddr::new(*ip, 0);
        match getnameinfo(&socket, flags) {
          Ok((longname, _)) => print!("{} ", longname),
          Err(_) => return Err("Failed to lookup socket"),
        }
      }

      Ok(())
    } else {
      Err("Error getting network interfaces")
    }
  } else if args.domain || args.fqdn || args.ip_address {
    const SOCK_DGRAM: i32 = 2;
    const AI_CANONNAME: i32 = 0x0002;
    let hints = AddrInfoHints {
      socktype: SOCK_DGRAM,
      flags: AI_CANONNAME,
      ..AddrInfoHints::default()
    };

    let hostname = gethostname()?;
    let sockets = match getaddrinfo(Some(&hostname), None, Some(hints)) {
      Ok(sockets) => sockets.collect::<std::io::Result<Vec<_>>>().unwrap(),  
      Err(_) => return Err("err")
    };

    let mut ip_address: Vec<String> = Vec::new();
    let mut p = String::new();

    for socket in sockets {
      ip_address.push(socket.sockaddr.ip().to_string());
      if socket.canonname.is_some() {
        p = socket.canonname.unwrap();
      }
    }

    if args.domain {
      if let Some((_, p)) = p.split_once('.') {
        println!("{p}");
      }
    } else if args.fqdn {
      println!("{p}");
    } else if args.ip_address {
      for ip in ip_address {
        println!("{ip} ");
      }
      println!();
    }

    Ok(())
  } else if args.short {
    let hostname = gethostname()?;
    let hostname =
      hostname
      .split('.')
      .collect::<Vec<&str>>();

    match hostname.get(0) {
      Some(hostname) => {
        println!("{hostname}");
        Ok(())
      },
      None => Err("hostname -short error")
    }
  } else if args.nis {
    let domainname = getdomainname()?;
    println!("{domainname}");

    Ok(())
  }
  else {
    let hostname = gethostname()?;
    println!("{hostname}");
    Ok(())
  }
}

fn main() {
  let args = Args::parse();

  run(args).unwrap_or_else(|err| {
    eprintln!("{err}");
    process::exit(1);
  })
}

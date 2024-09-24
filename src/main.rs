use clap::Parser;
use std::{fs, net::Ipv6Addr, process, ptr, mem};
use rhostname::{gethostname, getdomainname, sethostname, getnameinfo, dispnamealias};
use dns_lookup::{AddrInfoHints, getaddrinfo};

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
      .expect("No such file or directory.");
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

    dispnamealias();
    Ok(())

  } else if args.all_fqdns || args.all_ip_address {

    unsafe {
      use libc::{NI_NUMERICHOST, NI_NAMEREQD, IFF_LOOPBACK, AF_INET, AF_INET6};

      let mut ifap = ptr::null_mut();
      let mut ifa = ptr::null_mut();
      let flags = if args.all_ip_address { NI_NUMERICHOST } else { NI_NAMEREQD };

      if libc::getifaddrs(&mut ifap) != 0 {
        return Err("getifaddrs failed.");
      }
      
      loop {
        ifa = if ifa == ptr::null_mut() { ifap } else { (*ifa).ifa_next };

        if ifa.is_null() {
          break;
        }
        if (*ifa).ifa_addr.is_null() {
          continue;
        }
        if (*ifa).ifa_flags & IFF_LOOPBACK as u32 != 0 {
          continue;
        }
        if (*ifa).ifa_flags & libc::IFF_UP as u32 == 0 {
          continue;
        }

        let family = (*(*ifa).ifa_addr).sa_family;
        if family != AF_INET as u16 && family != AF_INET6 as u16 {
          continue;
        }

        let addrlen = if family == AF_INET as u16 { mem::size_of::<libc::sockaddr_in>() } else { mem::size_of::<libc::sockaddr_in6>() };

        if family == AF_INET6 as u16 {
          let sa_in6 = (*ifa).ifa_addr as *const libc::sockaddr_in6;
          let ipv6 = Ipv6Addr::from((*sa_in6).sin6_addr.s6_addr);

          if (ipv6.segments()[0] & 0xffc0) == 0xfe80 {
            continue;
          }
        }

        match getnameinfo((*ifa).ifa_addr, addrlen as u32, flags) {
          Ok(name) => print!("{name} "),
          Err(_) => continue
        }
      }

      println!();
      libc::freeifaddrs(ifap);
    }

    Ok(())

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
      Err(_) => return Err("getaddrinfo failed.")
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
      println!("{}", ip_address.join(" "));
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
      None => {
        println!("");
        Ok(())
      }
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
    eprintln!("hostname: {err}");
    process::exit(1);
  });
}

use clap::Parser;
use std::{fs, process};
use uthostname::{gethostname,sethostname};

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
  } else if args.all_fqdns {
    Ok(())
  } else if args.domain {
    Ok(())
  } else if args.fqdn {
    Ok(())
  } else if args.ip_address {
    Ok(())
  } else if args.all_ip_address {
    Ok(())
  } else if args.short {
    Ok(())
  } else if args.nis {
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

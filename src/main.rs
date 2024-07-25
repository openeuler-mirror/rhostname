use clap::Parser;
use libc::sethostname;
use std::ffi::CString;
use std::io::{Error,ErrorKind};
use std::fs;

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

fn get_hostname(args: Args) -> String {
  if args.alias {

  } else if args.all_fqdns {

  } else if args.domain {

  } else if args.fqdn {

  } else if args.ip_address {

  } else if args.all_ip_address {

  } else if args.short {

  } else if args.nis {

  }

  "".to_string()
}

fn set_hostname(name: String) {
  let len = name.len();
  let name = CString::new(name).unwrap();
  let result = unsafe { sethostname(name.as_ptr(), len)};

  println!("{:?}", result);

  if result != 0 {
    println!("{:?}", Error::last_os_error().kind());

    match Error::last_os_error().kind() {
      // EPERM
      ErrorKind::PermissionDenied => panic!(),

      _ => {}
    };
  }
}

fn main() {
  let args = Args::parse();
  print!("{:?}", args);

  if args.alias
    || args.all_fqdns
    || args.domain
    || args.fqdn
    || args.ip_address
    || args.all_ip_address
    || args.short
    || args.nis
  {
    println!("{}", get_hostname(args));
  } else if let Some(hostname) = args.boot {
    set_hostname(hostname);
  } else if let Some(filepath) = args.file {
    let contents = fs::read_to_string(filepath)
      .expect("File not exist.");
    let hostname = contents.split('\n')
      .collect::<Vec<&str>>()
      .iter()
      .copied()
      .filter(|&line| line != "" && !line.starts_with("#"))
      .collect::<Vec<&str>>();

    match hostname.get(0) {
      Some(hostname) => set_hostname(hostname.to_string()),
      None => set_hostname(String::from("")),
    };
  } else if let Some(hostname) = args.hostname {
    set_hostname(hostname);
  }
}

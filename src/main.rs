use clap::Parser;
use std::fs;
use crate::libc_wrapper::{gethostname,getdomainname,sethostname};

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

pub mod libc_wrapper {
  use std::ffi::CString;

  pub fn gethostname() -> Result<String, &'static str> {
    // > FROM `man gethostname`
    // SUSv2 guarantees that "Host names are limited to 255 bytes".  POSIX.1 guarantees that "Host names (not includ‐
    // ing the terminating null byte) are limited to HOST_NAME_MAX bytes".  On Linux, HOST_NAME_MAX is  defined  with
    // the value 64, which has been the limit since Linux 1.0 (earlier kernels imposed a limit of 8 bytes).

    let mut buf = vec![0u8; 255];
    let result = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

    if result != 0 {
      // EFAULT name is an invalid address.
      // ENAMETOOLONG (glibc gethostname()) len is smaller than the actual size. (Before version 2.1, glibc uses EINVAL for this case.)
      Err("Something went wrong.")
    }
    else {
      match buf.iter().position(|&x| x == 0) {
        Some(end) => {
          buf.resize(end, 0);
          Ok(CString::new(buf).unwrap().to_str().unwrap().to_string())
        },
        None => Err("Hostname is too large to fit."),
      }
    }
  }

  pub fn getdomainname() -> Result<String, &'static str> {
    // Since Linux 1.0, the limit on the length of a domain name, including the terminating null byte, is  64  bytes.
    // In older kernels, it was 8 bytes.

    let mut buf = vec![0u8; 255];
    let result = unsafe { libc::getdomainname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

    if result != 0 {
      Err("Something went wrong.")
    }
    else {
      match buf.iter().position(|&x| x == 0) {
        Some(end) => {
          buf.resize(end, 0);
          Ok(CString::new(buf).unwrap().to_str().unwrap().to_string())
        },
        None => Err("Domain name is too large to fit."),
      }
    }
  }

  pub fn sethostname(name: String) -> Result<(), &'static str> {
    let len = name.len();
    let name = CString::new(name).unwrap();
    let result = unsafe { libc::sethostname(name.as_ptr(), len)};

    if result != 0 {
      // println!("{:?}", Error::last_os_error().kind());

      // match Error::last_os_error().kind() {
      //   // EPERM
      //   ErrorKind::PermissionDenied => panic!(),

      //   _ => {}
      // };
      Err("Something went wrong.")
    }
    else {
      Ok(())
    }
  }
}

fn main() {
  let args = Args::parse();

  if let Some(hostname) = args.boot {
    sethostname(hostname);
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

    sethostname(hostname.to_string());
  } else if let Some(hostname) = args.hostname {
    sethostname(hostname);
  } else if args.alias {

  } else if args.all_fqdns {

  } else if args.domain {

  } else if args.fqdn {

  } else if args.ip_address {

  } else if args.all_ip_address {

  } else if args.short {

  } else if args.nis {

  }
}

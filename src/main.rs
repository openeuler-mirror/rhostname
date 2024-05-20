use clap::Parser;
use dns_lookup::{getaddrinfo, getnameinfo, AddrInfoHints, LookupErrorKind};
use libc::{getdomainname, gethostname, setdomainname, sethostname, sysconf, _SC_HOST_NAME_MAX};
use local_ip_address::list_afinet_netifas;
use std::ffi::{CString, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::net::{IpAddr, SocketAddr};
use std::os::unix::ffi::OsStringExt;
use std::process::exit;
use std::string::String;

pub const SOCK_DGRAM: i32 = 2;
pub const AI_CANONNAME: i32 = 0x0002;

#[derive(Debug, PartialEq)]
pub enum TypeT {
    DEFAULT,
    DNS,
    FQDN,
    SHORT,
    IP,
    NIS,
    NisDef,
    AllFqdns,
    AllIps,
}

#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
struct HostnameArgs {
    #[arg(short = 'A', long, help = "all long host names (FQDNs)")]
    all_fqdns: bool,

    #[arg(short = 'b', long, help = "set default hostname if none available")]
    boot: bool,

    #[arg(short = 'd', long, help = "DNS domain name")]
    domain: bool,

    #[arg(short = 'f', long = "fqdn, --long", help = "long host name (FQDN)")]
    fqdn: bool,

    #[arg(
        short = 'F',
        long,
        help = "read host name or NIS domain name from given file"
    )]
    file: Option<String>,

    #[arg(short = 'i', long, help = "addresses for the host name")]
    ip_address: bool,

    #[arg(short = 'I', long, help = "all addresses for the host")]
    all_ip_addresses: bool,

    #[arg(short = 's', long, help = "short host name")]
    short: bool,

    #[arg(short = 'y', long = "yp, --nis", help = "NIS/YP domain name")]
    yp: bool,

    hostname: Option<String>,
}

impl HostnameArgs {
    pub fn args_type(&self) -> char {
        if self.domain {
            return 'd';
        }
        if self.fqdn {
            return 'f';
        }
        if self.all_fqdns {
            return 'A';
        }
        if self.ip_address {
            return 'i';
        }
        if self.all_ip_addresses {
            return 'I';
        }
        if self.short {
            return 's';
        }
        if self.yp {
            return 'y';
        }
        if self.file.is_some() {
            return 'F';
        }
        if self.boot {
            return 'b';
        }
        '?'
    }
}


fn main() {
    let mut files: &Option<String> = &None;
    let mut type_d: TypeT = TypeT::DEFAULT;
    let opts = HostnameArgs::parse();
    let mut flag: i32 = 0;

    match opts.args_type() {
        'd' => {
            type_d = TypeT::DNS;
        }
        'f' => {
            type_d = TypeT::FQDN;
        }
        'A' => {
            type_d = TypeT::AllFqdns;
        }
        'i' => {
            type_d = TypeT::IP;
        }
        'I' => {
            type_d = TypeT::AllIps;
        }
        's' => {
            type_d = TypeT::SHORT;
        }
        'y' => {
            flag = 1;
            type_d = TypeT::NisDef;
        }
        'b' => {
            flag = 1;
            type_d = TypeT::DEFAULT;
        }
        'F' => {
            files = &opts.file;
        }
        _ => {}
    }
    let mut hostname = match opts.hostname {  
        Some(hostname) => {
            flag = 1;
            hostname
        }  
        None => String::new(),  
    };

    if !hostname.is_empty() && flag == 0 {
        usage();
    }

    if Some(files) != Some(&None) {
        hostname = read_file(files.clone().unwrap());
    }

    if !hostname.is_empty() {
        set_name(type_d, hostname);
    } else {
        show_name(type_d);
    }
}

pub fn show_name(domaintype: TypeT) {
    let mut p: String = String::new();
    let name = localhost();

    match domaintype {
        TypeT::DEFAULT => {
            println!("{}", name);
        }
        TypeT::SHORT => {
            p = name.to_string();
            let p: Vec<&str> = p.split('.').collect();
            println!("{}", p[0]);
        }
        TypeT::NIS => {
            println!("{}", localdomain());
        }
        TypeT::NisDef => {
            println!("{}", localnisdomain());
        }
        TypeT::AllIps | TypeT::AllFqdns => {
            let network_interfaces = list_afinet_netifas();

            if let Ok(network_interfaces) = network_interfaces {
                for (_, ip) in network_interfaces.iter() {
                    if !ip.is_ipv4() && !ip.is_ipv6()
                        || ip.is_loopback()
                        || ip.to_string().is_empty()
                    {
                        continue;
                    }
                    if ip.is_ipv6() {
                        if ip.is_multicast() {
                            continue;
                        }
                        let ips = ip.to_string();
                        let ips: Vec<&str> = ips.split(':').collect();
                        if ips[0] == "fe80" {
                            continue;
                        }
                    }

                    let ip: IpAddr = ip.to_string().parse().unwrap();
                    let socket: SocketAddr = (ip, 0).into();
                    let mut flags = 0;
                    if domaintype == TypeT::AllIps {
                        flags = 1;
                    }

                    match getnameinfo(&socket, flags) {
                        Ok((longname, _)) => print!("{} ", longname),
                        Err(e) => println!("Failed to lookup socket {:?}", e),
                    };
                }
                println!(" ");
            }
        }
        _ => {
            let hints = AddrInfoHints {
                socktype: SOCK_DGRAM,
                flags: AI_CANONNAME,
                ..AddrInfoHints::default()
            };

            let sockets = match getaddrinfo(Some(&name), None, Some(hints)) {
                Ok(sockets) => sockets.collect::<std::io::Result<Vec<_>>>().unwrap(),  
                Err(e) => { 
                    match e.kind() {
                        LookupErrorKind::NoName => println!("Name or service not known"),
                        _ => println!("Err")
                    }
                    return;
                } 
            };

            let mut ip_addresses: Vec<String> = Vec::new();

            for socket in sockets {
                let ip = socket.sockaddr.ip().to_string();
                ip_addresses.push(ip);
                if socket.canonname.is_some() {
                    p = socket.canonname.unwrap();
                }
            }

            match domaintype {
                TypeT::IP => {
                    for ip in ip_addresses {
                        print!("{} ", ip);
                    }
                    println!(" ");
                }
                TypeT::DNS => {
                    if p.contains('.') {
                        let (_, p) = p.split_once('.').unwrap();
                        println!("{} ", p);
                    }
                }
                TypeT::FQDN => {
                    println!("{} ", p);
                }
                _ => {}
            }
        }
    }
}

pub fn localhost() -> String {
    let buf_len = unsafe { sysconf(_SC_HOST_NAME_MAX) as libc::size_t };
    let mut buf = vec![0u8; buf_len];

    let myerror = unsafe { gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf_len) };
    if myerror != 0 {
        println!("failed to get hostname");
        exit(1);
    }

    parse_string(buf)
}

pub fn localdomain() -> String {
    let buf_len = unsafe { sysconf(_SC_HOST_NAME_MAX) as libc::size_t };
    let mut buf = vec![0u8; buf_len];

    let myerror = unsafe { getdomainname(buf.as_mut_ptr() as *mut libc::c_char, buf_len) };

    if myerror != 0 {
        println!("failed to get domainname");
        exit(1);
    }

    parse_string(buf)
}

pub fn localnisdomain() -> String {
    let mut buf = vec![0u8; 1025];

    let myerror = unsafe { getdomainname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
    let buf = parse_string(buf);

    if myerror != 0 || buf == "(none)" {
        println!("Local domain name not set");
        exit(1);
    }

    buf
}

pub fn parse_string(mut buf: Vec<u8>) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    buf.resize(end, 0);

    let buf = OsString::from_vec(buf).to_str().unwrap().to_string();

    buf
}

pub fn set_name(domaintype: TypeT, name: String) {
    let mut name_len: usize = name.len();

    match domaintype {
        TypeT::DEFAULT => {
            let name = name.trim().to_string();

            if check_name(name.clone()) != 1 {
                println!("the specified hostname is invalid");
                exit(1);
            }

            name_len = name.len();
            let name_str = CString::new(name).unwrap();
            let result = unsafe { sethostname(name_str.as_ptr(), name_len) };

            if result != 0 {
                match Error::last_os_error().kind() {
                    ErrorKind::PermissionDenied => {
                        println!("you must be root to change the host name");
                        exit(1);
                    }
                    ErrorKind::InvalidInput => {
                        println!("name too long");
                        exit(1);
                    }
                    _ => {}
                };
            }
        }
        TypeT::NIS | TypeT::NisDef => {
            let name_str = CString::new(name).unwrap();
            let result = unsafe { setdomainname(name_str.as_ptr(), name_len) };

            if result != 0 {
                match Error::last_os_error().kind() {
                    ErrorKind::PermissionDenied => {
                        println!("you must be root to change the domain name");
                        exit(1);
                    }
                    ErrorKind::InvalidInput => {
                        println!("name too long");
                        exit(1);
                    }
                    _ => {}
                };
            }
        }
        _ => usage(),
    }
}

pub fn check_name(name: String) -> i64 {
    let len = name.len();

    let next = name.chars().next().unwrap();
    let last = name.chars().last().unwrap();
    if len == 0 || !next.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
        return 0;
    }

    let mut has_dot = false;
    for ch in name.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '-' && ch != '.' {
            return 0;
        }
        if ch == '.' {
            if has_dot {
                return 0;
            }
            has_dot = true;
        }
    }

    1
}

pub fn read_file(filename: String) -> String {
    let mut buf: String = String::new();

    let file = match File::open(&filename) {
        Err(_) => {
            println!("{}: No such file or directory", filename);
            exit(0);
        }
        Ok(file) => file,
    };

    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        if line.starts_with('\n') || line.starts_with('#') {
            continue;
        }
        buf = line.replace('\n', "");
    }

    buf
}

pub fn usage() {
    println!(
        "Usage: hostname [OPTIONS] [NAME]\n
Arguments:[NAME]\n
Options:
    -A, --all-fqdns         all long host names (FQDNs)
    -b, --boot              set default hostname if none available
    -d, --domain            DNS domain name
    -f, --fqdn, --long      long host name (FQDN)
    -F, --file <FILE>       read host name or NIS domain name from given file
    -i, --ip-address        addresses for the host name
    -I, --all-ip-addresses  all addresses for the host
    -s, --short             short host name 
    -y, --yp, --nis         NIS/YP domain name
    -h, --help              Print help"
    );
    exit(-1);
}


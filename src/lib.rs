use std::io::Error;
use std::ffi::{CString, c_char};

extern "C" {
  fn hostname_alias(name: *const c_char);
}

fn u8_to_string(mut v: Vec<u8>) -> Option<String> {
  match v.iter().position(|&x| x == 0) {
    Some(end) => {
      v.resize(end, 0);
      Some(CString::new(v).unwrap().to_str().unwrap().to_string())
    },
    None => None
  }
}

pub fn gethostname() -> Result<String, &'static str> {
  // > FROM `man gethostname`
  // SUSv2 guarantees that "Host names are limited to 255 bytes".  POSIX.1 guarantees that "Host names (not includ‐
  // ing the terminating null byte) are limited to HOST_NAME_MAX bytes".  On Linux, HOST_NAME_MAX is  defined  with
  // the value 64, which has been the limit since Linux 1.0 (earlier kernels imposed a limit of 8 bytes).

  let mut buf = vec![0u8; 255];
  let ret = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

  if ret != 0 {
    Err("Unknown error.")
  }
  else {
    // > FROM `man gethostname`
    // If the null-terminated hostname is too large to fit, then the name is truncated, and no error is returned
    // (but see NOTES below).  POSIX.1 says that if such truncation occurs, then it is unspecified whether the
    // returned buffer includes a terminating null byte.
    match u8_to_string(buf) {
      Some(name) => Ok(name),
      None => Err("Hostname is too large to fit."),
    }
  }
}

pub fn getdomainname() -> Result<String, &'static str> {
  // Since Linux 1.0, the limit on the length of a domain name, including the terminating null byte, is  64  bytes.
  // In older kernels, it was 8 bytes.

  let mut buf = vec![0u8; 255];
  let ret = unsafe { libc::getdomainname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

  if ret != 0 {
    Err("Unknown error.")
  }
  else {
    // getdomainname() returns the null-terminated domain name in the character array name, which has a length of len
    // bytes.  If the null-terminated domain name requires more than len bytes, getdomainname() returns the first len
    // bytes (glibc) or gives an error (libc).
    match u8_to_string(buf) {
      Some(name) => Ok(name),
      None => Err("Domain name is too large to fit."),
    }
  }
}

fn check_hostname(name: String) -> bool {
  let name = name.as_bytes();

  if name.len() == 0 || !name[0].is_ascii_alphanumeric() || !name[name.len() - 1].is_ascii_alphanumeric() {
    return false;
  }

  for i in 1..(name.len() - 1) {
    if !name[i].is_ascii_alphanumeric() && name[i] != b'-' && name[i] != b'.' {
      return false;
    }
    if name[i] == b'-' && (name[i - 1] == b'.' || name[i + 1] == b'.') {
      return false;
    }
    if name[i] == b'.' && name[i - 1] == b'.' {
      return false;
    }
  }

  true
}

pub fn sethostname(name: String) -> Result<(), &'static str> {
  if check_hostname(name.clone()) == false {
    return Err("the specified hostname is invalid")
  }
  
  let len = name.len();
  let name = CString::new(name).unwrap();
  let ret = unsafe { libc::sethostname(name.as_ptr(), len)};

  if ret != 0 {
    match Error::last_os_error().raw_os_error() {
      // > FROM `man gethostname`
      // EFAULT name is an invalid address.
      // EINVAL len is negative or, for sethostname(), len is larger than the maximum allowed size.
      // ENAMETOOLONG
      //        (glibc gethostname()) len is smaller than the actual size.  (Before version 2.1, glibc uses EINVAL  for
      //        this case.)
      // EPERM  For  sethostname(),  the caller did not have the CAP_SYS_ADMIN capability in the user namespace associ‐
      // ated with its UTS namespace (see namespaces(7)).

      Some(err) => match err {
        // libc::EFAULT => Err(""),
        // libc::ENAMETOOLONG => Err(""),
        libc::EINVAL => Err("name too long"),
        libc::EPERM => Err("you must be root to change the host name"),
        _ => Err("Unknown error."),
      },
      None => Err("Unknown error."),
    }
  }
  else {
    Ok(())
  }
}

pub fn getnameinfo(sa: *const libc::sockaddr, salen: libc::socklen_t, flags: libc::c_int) -> Result<String, &'static str> {
  let mut host = vec![0u8; libc::NI_MAXHOST as usize];
  let ret = unsafe { libc::getnameinfo(sa, salen, host.as_mut_ptr() as *mut libc::c_char, host.len() as u32, std::ptr::null_mut(), 0, flags) };

  if ret != 0 {
    // > FROM `man getnameinfo`
    // The  gai_strerror(3)  function translates these error codes to a human readable string, suitable for error re‐
    // porting.
    if flags != libc::NI_NAMEREQD && ret != libc::EAI_NONAME {
      unsafe { libc::gai_strerror(ret); }
      std::process::exit(1);
    }
    Err("Unknown error.")
  }
  else {
    Ok(u8_to_string(host).unwrap())
  }
}

pub fn dispnamealias() {
  let name = CString::new(gethostname().unwrap()).unwrap();
  let name = name.as_bytes_with_nul();

  unsafe { hostname_alias(name.as_ptr() as *const c_char) };
}
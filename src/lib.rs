use std::ffi::CString;

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
    // EFAULT name is an invalid address.
    // ENAMETOOLONG (glibc gethostname()) len is smaller than the actual size. (Before version 2.1, glibc uses EINVAL for this case.)
    Err("Something went wrong.")
  }
  else {
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
    Err("Something went wrong.")
  }
  else {
    match u8_to_string(buf) {
      Some(name) => Ok(name),
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

pub fn getnameinfo(sa: *const libc::sockaddr, salen: libc::socklen_t, flags: libc::c_int) -> Result<String, &'static str> {
  let mut host = vec![0u8; libc::NI_MAXHOST as usize];
  let ret = unsafe { libc::getnameinfo(sa, salen, host.as_mut_ptr() as *mut libc::c_char, host.len() as u32, std::ptr::null_mut(), 0, flags) };

  if ret != 0 {
    Err("")
  }
  else {
    Ok(u8_to_string(host).unwrap())
  }
}
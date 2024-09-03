use std::process::Command;

fn test_sudo(args: Vec<&str>, name: &str) {
  let args = [vec!["./target/debug/uthostname"], args.clone()].concat();

  Command::new("sudo")
    .args(args)
    .output()
    .expect("failed to execute process");

  let output = 
    Command::new("./target/debug/uthostname")
      .output()
      .expect("failed to execute process");
  
  assert_eq!(name, std::str::from_utf8(&output.stdout).unwrap());
}

fn test(args: Vec<&str>) {
  let lhs =
    Command::new("hostname")
      .args(args.clone())
      .output()
      .expect("failed to execute process");
  let rhs =
    Command::new("./target/debug/uthostname")
      .args(args.clone())
      .output()
      .expect("failed to execute process");

  // assert_eq!(lhs.status, rhs.status);
  assert_eq!(std::str::from_utf8(&lhs.stdout).unwrap(), std::str::from_utf8(&rhs.stdout).unwrap());
  assert_eq!(std::str::from_utf8(&lhs.stderr).unwrap(), std::str::from_utf8(&rhs.stderr).unwrap());
}

#[test]
fn test_get() {
  test(vec![""]);
}

#[test]
fn test_set() {
  test_sudo(vec!["test"], "test\n");
  test(vec![""]);
  test_sudo(vec!["localhost"], "localhost\n");
  test(vec![""]);
}

#[test]
fn test_alias() {
  test(vec!["-a"]);
}

#[test]
fn test_all_fqdns() {
  test(vec!["-A"]);
}

#[test]
fn test_bool() {
  test_sudo(vec!["-b", "test"], "test\n");
  test(vec![""]);
  test_sudo(vec!["-b", "localhost"], "localhost\n");
  test(vec![""]);
}

#[test]
fn test_domain() {
  test(vec!["-d"]);
}

#[test]
fn test_fqdn() {
  test(vec!["-f"]);
}

#[test]
fn test_file() {
  test_sudo(vec!["-F", "./tests/hostname"], "insorker\n");
  test(vec![""]);
}

#[test]
fn test_ip_address() {
  test(vec!["-i"]);
}

#[test]
fn test_all_ip_address() {
  test(vec!["-I"]);
}

#[test]
fn test_short() {
  test(vec!["-s"]);
}

#[test]
fn test_yp() {
  test(vec!["-y"]);
}

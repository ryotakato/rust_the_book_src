use std::fmt::{self, Display, Formatter };


#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl Display for IpAddrKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            IpAddrKind::V4(ip1, ip2, ip3, ip4) => write!(f, "V4 = ({}, {}, {}, {})", ip1, ip2, ip3, ip4),
            IpAddrKind::V6(ip) => write!(f, "V6 = ({})", ip),
        }?;
        Ok(())
    }
}



fn main() {

    println!("== match ========");
    for i in 0..3 {
      let ip_addr = assign(i);
      match ip_addr {
          Some(ip_addr_kind) => println!("{}", ip_addr_kind),
          None => println!("assign failed")
      }
    }

    println!("== if let ========");
    for i in 0..3 {
      let ip_addr = assign(i);
      if let Some(ip_addr_kind) = ip_addr {
          println!("{}", ip_addr_kind);
      }
    }
}


fn assign(rand: u8) -> Option<IpAddrKind> {
    match rand {
        1 => Some(IpAddrKind::V4(127, 0, 0, 1)),
        2 => Some(IpAddrKind::V6(String::from("::1"))),
        _ => None,
    }
}

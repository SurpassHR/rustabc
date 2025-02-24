#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn print(&self) {
        match self {
            IpAddrKind::V4(a, b, c, d) => {
                println!("ipv4 addr: {}.{}.{}.{}", a, b, c, d);
            }
            IpAddrKind::V6(addr) => {
                println!("ipv5 addr: {}", addr);
            }
        }
    }
}

fn main() {
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let ip_v6_addr: IpAddrKind = IpAddrKind::V6(String::from("fe80::215:5dff:fe06:a4f5"));

    // first print method
    println!("ip v4 addr: {:?}, ip v6 addr: {:?}", localhost, ip_v6_addr);

    // second print method
    localhost.print();
    ip_v6_addr.print();
}

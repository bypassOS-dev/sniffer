use std::net::Ipv4Addr;

use nfq::{Queue, Verdict};
use pnet::packet::ipv4::Ipv4Packet;
#[tokio::main]
async fn main() {
    let mut queue = Queue::open().unwrap();
    queue.bind(0).unwrap();

    loop {
        let mut msg = queue.recv().unwrap();
        let payload = msg.get_payload();

        if let Some(ipv4) = Ipv4Packet::new(payload) {
            let src_ip = ipv4.get_source();
            let dest_ip = ipv4.get_destination();
            let ttl = ipv4.get_ttl();
        }
        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg).unwrap();
    }
}
async fn create_wrapper(ip_addr_src: Ipv4Addr, ip_addr_dest: Ipv4Addr, ttl: u8) {
    //================================================
    let ip_addr_str = ip_addr_src.to_string();
    let ip_addr_len = ip_addr_str.len();
    let do_space = 15 - ip_addr_len;
    //=================================================
    println!(" ==========================================");
    println!("|| From: {}                 {:do_space$}||", ip_addr_src, do_space);
    println!("|| To: {}                                  ||", ip_addr_dest);
    println!("|| TTL: {}                                 ||", ttl);
    println!("||");
    println!("||");
    println!("||");
    println!("||");
    println!("||");
    println!(" ============================================")
}

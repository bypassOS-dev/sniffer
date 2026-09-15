use nfq::{Queue, Verdict};
use pnet::packet::ipv4::Ipv4Packet;
use std::net::Ipv4Addr;
use tokio::process::Command;
#[tokio::main]
async fn main() {
    let status = Command::new("iptables")
        .arg("-A")
        .arg("OUTPUT")
        .arg("-p")
        .arg("tcp")
        .arg("--dport")
        .arg("443")
        .arg("-j")
        .arg("NFQUEUE")
        .arg("--queue-num")
        .arg("0")
        .status()
        .await
        .expect("Executing error");

    println!("Status: {status}");
    loop {
        tokio::select! {
            _ = like_main() => {

            }
            _ = tokio::signal::ctrl_c() => {
                let _status = Command::new("iptables")
                    .arg("-D")
                    .arg("OUTPUT")
                    .arg("-p")
                    .arg("tcp")
                    .arg("-d")
                    .arg("172.217.76.93")
                    .arg("--dport")
                    .arg("443")
                    .arg("-j")
                    .arg("NFQUEUE")
                    .arg("--queue-num")
                    .arg("0")
                    .status()
                    .await
                    .expect("Executing error");

                    std::process::exit(0);
                }
            }
    }
}
async fn create_wrapper(ip_addr_src: Ipv4Addr, ip_addr_dest: Ipv4Addr, ttl: u8) {
    //================================================
    let ip_addr_str = ip_addr_src.to_string();
    let ip_addr_len = ip_addr_str.len();
    let do_space = 15 - ip_addr_len;
    //=================================================
    let ip_addr_str = ip_addr_dest.to_string();
    let ip_addr_len = ip_addr_str.len();
    let do_space1 = 15 - ip_addr_len;
    //=================================================
    let ttl = ttl.to_string();
    let ttl_len = ttl.len();
    let do_space2 = 3 - ttl_len;
    //=================================================
    println!(" ==========================================");
    println!("|| From: {}                 {:do_space$}||",ip_addr_src, do_space);
    println!("|| To: {}                   {:do_space1$}||",ip_addr_dest, do_space1);
    println!("|| TTL: {}                              {:do_space2$}||",ttl, do_space2);
    println!("||");
    println!("||");
    println!("||");
    println!("||");
    println!("||");
    println!(" ============================================")
}
async fn like_main() {
    let mut queue = Queue::open().unwrap();
    queue.bind(0).unwrap();

    loop {
        let mut msg = queue.recv().unwrap();
        let payload = msg.get_payload();

        if let Some(ipv4) = Ipv4Packet::new(payload) {
            let src_ip = ipv4.get_source();
            let dest_ip = ipv4.get_destination();
            let ttl = ipv4.get_ttl();

            create_wrapper(src_ip, dest_ip, ttl).await;
        }
        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg).unwrap();
    }
}

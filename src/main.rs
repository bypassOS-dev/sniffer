use nfq::{Queue, Verdict};
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::{TcpPacket, TcpFlags};
use std::net::Ipv4Addr;
use pnet::packet::ip::IpNextHeaderProtocols;
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
async fn create_wrapper(
        ip_addr_src: Ipv4Addr, 
        src_port: u16, 

        ip_addr_dest: Ipv4Addr, 
        dest_port: u16,

        ttl: u8
    ) 
    {
    //================================================
    let ip_addr_str = ip_addr_src.to_string();
    let ip_addr_len = ip_addr_str.len();
    let do_space = 15 - ip_addr_len;
    //============
    let source_port = src_port.to_string();
    let source_port_lenght = source_port.len();
    let do_space3 = 5 - source_port_lenght;
    //=================================================
    let ip_addr_str = ip_addr_dest.to_string();
    let ip_addr_len = ip_addr_str.len();
    let do_space1 = 15 - ip_addr_len;
    //===========
    let dest_port  = dest_port.to_string();
    let dest_porrt_lenght = dest_port.len();
    let do_space4 = 5 - dest_porrt_lenght;
    //=================================================
    let ttl = ttl.to_string();
    let ttl_len = ttl.len();
    let do_space2 = 3 - ttl_len;
    //=================================================
    println!(" ==========================================");
    println!("|| From: {}            {:do_space$}:{:do_space3$} ||",ip_addr_src, do_space, do_space3);
    println!("|| To: {}                   {:do_space1$}: {:do_space4$} ||",ip_addr_dest, do_space1, do_space4);
    println!("|| TTL: {}                         {:do_space2$} ||",ttl, do_space2);
    println!("|| QWQ:                                     ||");
    println!("||                                          ||");
    println!("||                                          ||");
    println!("||                                          ||");
    println!("||                                          ||");
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

            if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                if let Some(tcp_packet) = TcpPacket::new(ipv4.payload()) {
                    let src_port = tcp_packet.get_source();
                    let dest_port = tcp_packet.get_destination();
                    create_wrapper(src_ip, src_port,  dest_ip, dest_port, ttl).await;
                }
            }   
        }
        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg).unwrap();
    }
}

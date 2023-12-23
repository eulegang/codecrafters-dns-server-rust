// Uncomment this block to pass the first stage
// use std::net::UdpSocket;

use std::net::{Ipv4Addr, SocketAddrV4};

use clap::Parser;
use tokio::net::UdpSocket;

use fmt::Bincode;

mod fmt;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long)]
    resolver: Option<SocketAddrV4>,
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let cli = Cli::parse();
    eprintln!("Cli: {cli:?}");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053")
        .await
        .expect("Failed to bind to address");

    let forwarder = if let Some(addr) = cli.resolver {
        let sock = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("failed to bind forwarder to address");
        sock.connect(addr)
            .await
            .expect("could not connect udp socket");

        Some(sock)
    } else {
        None
    };

    let mut in_buf = [0; 512];
    let mut out_buf = Vec::with_capacity(512);

    'conn: loop {
        let (bytes, source) = match udp_socket.recv_from(&mut in_buf).await {
            Ok((size, source)) => {
                eprintln!("Received {} bytes from {}", size, source);
                (&in_buf[0..size], source)
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        };

        let packet = match fmt::Packet::decode(bytes) {
            Ok((_, packet)) => packet,
            Err(e) => {
                eprintln!("malformed packet: {e:?}");
                continue;
            }
        };

        if let Some(ref forwarder) = forwarder {
            let mut len = 0;

            while len < bytes.len() {
                let Ok(i) = forwarder.send(&bytes[len..]).await else {
                    continue 'conn;
                };
                len += i;
            }

            let Ok(_) = forwarder.recv_buf(&mut out_buf).await else {
                continue 'conn;
            };

            udp_socket
                .send_to(&out_buf, source)
                .await
                .expect("Failed to send response");
        } else {
            let packet = transform(packet);

            packet.encode(&mut out_buf);

            udp_socket
                .send_to(&out_buf, source)
                .await
                .expect("Failed to send response");
        }

        out_buf.clear();
    }
}

fn transform(mut packet: fmt::Packet) -> fmt::Packet {
    let mut answers = Vec::new();
    for q in &packet.questions {
        answers.push(fmt::Resource {
            name: q.name.clone(),
            ty: q.ty,
            class: q.class,
            ttl: 60,
            data: fmt::RData::from(Ipv4Addr::from([8, 8, 8, 8])),
        })
    }

    packet.header.an_count = answers.len() as u16;
    packet.answers = answers;

    packet.header.set_side(fmt::Side::Response);

    if packet.header.opcode() != 0 {
        packet.header.set_rcode(4);
    }

    packet
}

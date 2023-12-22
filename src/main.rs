// Uncomment this block to pass the first stage
// use std::net::UdpSocket;

use std::net::Ipv4Addr;

use tokio::net::UdpSocket;

use fmt::Bincode;

mod fmt;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053")
        .await
        .expect("Failed to bind to address");

    let mut in_buf = [0; 512];
    let mut out_buf = Vec::with_capacity(512);

    'conn: loop {
        match udp_socket.recv_from(&mut in_buf).await {
            Ok((size, source)) => {
                eprintln!("Received {} bytes from {}", size, source);
                let bytes = &in_buf[0..size];

                let packet = match fmt::Packet::decode(bytes) {
                    Ok((_, packet)) => packet,

                    Err(e) => {
                        eprintln!("malformed request header: {e:?}");

                        continue 'conn;
                    }
                };

                let packet = transform(packet);

                packet.encode(&mut out_buf);

                udp_socket
                    .send_to(&out_buf, source)
                    .await
                    .expect("Failed to send response");

                out_buf.clear();
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
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

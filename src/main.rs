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

                let Ok((mut body, req_header)) = fmt::Header::decode(bytes) else {
                    eprintln!("malformed request header");

                    continue 'conn;
                };

                let mut questions = Vec::new();
                for _ in 0..req_header.qd_count {
                    let Ok((b, q)) = fmt::Question::decode(body) else {
                        eprintln!("malformed question");

                        continue 'conn;
                    };

                    body = b;
                    questions.push(q);
                }

                let mut header = fmt::Header::default();
                header.id = req_header.id;
                header.qd_count = req_header.qd_count;
                header.an_count = 1;

                header.set_opcode(req_header.opcode());
                header.set_side(fmt::Side::Response);
                header.set_recursion_desired(req_header.recursion_desired());

                if req_header.opcode() != 0 {
                    header.set_rcode(4);
                }

                header.encode(&mut out_buf);

                let mut answers = Vec::new();

                for q in questions {
                    q.encode(&mut out_buf);

                    answers.push(fmt::Resource {
                        name: q.name.clone(),
                        ty: q.ty,
                        class: q.class,
                        ttl: 60,
                        data: Ipv4Addr::from([8, 8, 8, 8]).into(),
                    });
                }

                for a in answers {
                    a.encode(&mut out_buf);
                }

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

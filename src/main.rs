// Uncomment this block to pass the first stage
// use std::net::UdpSocket;

use tokio::net::UdpSocket;

mod fmt;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053")
        .await
        .expect("Failed to bind to address");

    let mut in_buf = [0; 512];
    let mut out_buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut in_buf).await {
            Ok((size, source)) => {
                eprintln!("Received {} bytes from {}", size, source);
                let bytes = &in_buf[0..size];

                let Ok((body, req_header)) = fmt::Header::parse(bytes) else {
                    eprintln!("malformed request header");

                    continue;
                };

                let mut header = fmt::Header::default();
                header.id = req_header.id;
                header.set_side(fmt::Side::Response);

                let len = header.write_to(&mut out_buf);

                udp_socket
                    .send_to(&out_buf[0..len], source)
                    .await
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

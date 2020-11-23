mod packets;
mod message;

use std::io;
use async_std::{
    task,
    net::{
        UdpSocket,
        SocketAddr,
    },
};

async fn send_ack(sock: &UdpSocket, peer: &SocketAddr, nonce: u16, missing_packets: u8) -> Result<usize, io::Error> {
    let buf = packets::AckPacket::encode(nonce, missing_packets);
    sock.send_to(buf.as_slice(), peer).await
}

async fn async_main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:22023").await?;
    println!("Listening on {}", socket.local_addr()?);

    let mut buf = vec![0u8; 1024*16]; // 16kb Buffer for incomming messages

    loop {
        let (recv, peer) = socket.recv_from(&mut buf).await?;
        match packets::parse_packet(&buf) {
            Ok((typ, packet)) => {
                println!("{:#?}", typ);

                match typ {
                    packets::PacketType::HELLO => {
                        let packet = packet.downcast::<packets::HelloPacket>().unwrap();
                        send_ack(&socket, &peer, packet.nonce, 128).await?;
                    },
                    packets::PacketType::RELIABLE => {
                        let packet = packet.downcast::<packets::ReliablePacket>().unwrap();
                        println!("{:#?}", packet.payload);
                        send_ack(&socket, &peer, packet.nonce, 128).await?;
                    },
                    _ => ()
                }
            },
            Err(err) => println!("{}", err.to_string()),
        };
    }
}

fn main() -> std::io::Result<()> {
    task::block_on(async_main())
}
use super::packets;
use crate::objects;

use async_std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::{convert::TryFrom, error::Error, io};

pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl Server {
    pub async fn new<A: ToSocketAddrs + std::fmt::Display>(addr: A) -> io::Result<Server> {
        let socket = UdpSocket::bind(&addr).await?;
        println!("Socket listening on addr {}...", addr);

        let buf = vec![0u8; 1024 * 16];

        Ok(Server { socket, buf })
    }

    pub async fn listen_blocking(&mut self) {
        loop {
            self.handle_message().await;
        }
    }

    async fn handle_message(&mut self) -> io::Result<()> {
        let (recv, peer) = self.socket.recv_from(&mut self.buf).await?;

        match packets::parse_packet(&self.buf) {
            Ok((typ, packet)) => {
                println!("{:#?}", typ);

                match typ {
                    packets::PacketType::HELLO => {
                        let packet = packet.downcast::<packets::HelloPacket>().unwrap();
                        self.send_ack(&peer, packet.nonce, 128).await?;
                    }
                    packets::PacketType::RELIABLE => {
                        let packet = packet.downcast::<packets::ReliablePacket>().unwrap();
                        // println!("{:#?}", packet.payload);
                        self.send_ack(&peer, packet.nonce, 128).await?;
                        match Self::parse_object(&packet) {
                            Ok(_) => (),
                            Err(err) => println!("{}", err),
                        }
                    }
                    packets::PacketType::PING => {
                        let packet = packet.downcast::<packets::PingPacket>().unwrap();
                        self.send_ack(&peer, packet.nonce, 128).await?;
                    }
                    _ => (),
                }
            }
            Err(err) => println!("{}", err.to_string()),
        };

        Ok(())
    }

    async fn send_ack(
        &self,
        peer: &SocketAddr,
        nonce: u16,
        missing_packets: u8,
    ) -> Result<usize, io::Error> {
        let buf = packets::AckPacket::encode(nonce, missing_packets);
        self.socket.send_to(buf.as_slice(), peer).await
    }

    fn parse_object(packet: &packets::ReliablePacket) -> Result<(), Box<dyn Error>> {
        let typ = match objects::RootMessageType::try_from(packet.payload[0].tag) {
            Ok(v) => v,
            Err(_) => return Err("invalid root object type".into()),
        };

        match typ {
            objects::RootMessageType::HOST_GAME => {
                let d = objects::GameOptionsData::parse(&packet.payload[0]);
                println!("{:#?}", d);
            }
            _ => return Err(format!("invalid root object type: {:#?}", typ).into()),
        }

        Ok(())
    }
}

use super::packets;
use crate::objects;

use async_std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::{collections::HashMap, convert::TryFrom, error::Error, io};

/// "Placeholder" struct for whatever else
/// might be useful in there later. :^)
pub struct Client {}

/// The main UDP server to maintain connections
// to the clients.
pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,

    connected_clients: HashMap<String, Client>,
}

impl Server {
    pub async fn new<A: ToSocketAddrs + std::fmt::Display>(addr: A) -> io::Result<Server> {
        let socket = UdpSocket::bind(&addr).await?;
        println!("Socket listening on addr {}...", addr);

        let buf = vec![0u8; 1024 * 4];
        let connected_clients: HashMap<String, Client> = HashMap::new();

        Ok(Server {
            socket,
            buf,
            connected_clients,
        })
    }

    pub async fn listen_blocking(&mut self) {
        loop {
            self.handle_message().await; // TODO: Handle result
        }
    }

    async fn handle_message(&mut self) -> io::Result<()> {
        let (_, peer) = self.socket.recv_from(&mut self.buf).await?;

        match packets::parse_packet(&self.buf) {
            Ok((typ, packet)) => {
                println!("{:#?}", typ);

                match typ {
                    packets::PacketType::HELLO => {
                        let packet = packet.downcast::<packets::HelloPacket>().unwrap();
                        self.send_ack(&peer, packet.nonce, 128).await?;
                        self.connected_clients.insert(peer.to_string(), Client {});
                    }
                    packets::PacketType::RELIABLE => {
                        let packet = packet.downcast::<packets::ReliablePacket>().unwrap();
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

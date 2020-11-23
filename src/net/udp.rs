use super::packets::*;
use crate::bufpool::BufPool;
use crate::objects;

use async_std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::{collections::HashMap, convert::TryFrom, error::Error, io};

/// "Placeholder" struct for whatever else
/// might be useful in there later. :^)
#[derive(Debug)]
pub struct Client {}

/// The main UDP server to maintain connections
/// to the clients.
pub struct Server {
    socket: UdpSocket,
    buf_pool: BufPool<u8>,

    connected_clients: HashMap<String, Client>,
}

impl Server {
    /// Creates a new Server instance binding to
    /// the given address.
    ///
    /// May result in an error when the socket
    /// bind fails.
    pub async fn new<A: ToSocketAddrs + std::fmt::Display>(addr: A) -> io::Result<Server> {
        let socket = UdpSocket::bind(&addr).await?;
        println!("Socket listening on addr {}...", addr);

        Ok(Server {
            socket,
            buf_pool: BufPool::new(1024 * 16),
            connected_clients: HashMap::new(),
        })
    }

    /// Starts the message handler loop waiting for
    /// and processing incomming messages blocking
    /// the current thread.
    pub async fn listen_blocking(&mut self) {
        loop {
            self.handle_message().await; // TODO: Handle result
        }
    }

    async fn handle_message(&mut self) -> io::Result<()> {
        let mut buf = self.buf_pool.take().await;
        let (_, peer) = self.socket.recv_from(&mut buf).await?;

        match parse_packet(&mut buf) {
            Ok((typ, packet)) => {
                println!("{:#?}", typ);

                match typ {
                    PacketType::HELLO => {
                        let packet = packet.downcast::<HelloPacket>().unwrap();
                        self.send_ack(&peer, packet.nonce, 128).await?;
                        self.connected_clients.insert(peer.to_string(), Client {});
                    }
                    PacketType::RELIABLE => {
                        let packet = packet.downcast::<ReliablePacket>().unwrap();
                        self.send_ack(&peer, packet.nonce, 128).await?;
                        match Self::parse_object(&packet) {
                            Ok(_) => (),
                            Err(err) => println!("{}", err),
                        }
                    }
                    PacketType::PING => {
                        let packet = packet.downcast::<PingPacket>().unwrap();
                        self.send_ack(&peer, packet.nonce, 128).await?;
                    }
                    PacketType::DISCONNECT => {
                        self.connected_clients.remove(&peer.to_string()).unwrap();
                    }
                    _ => (),
                }
            }
            Err(err) => println!("{}", err.to_string()),
        };

        self.buf_pool.back(buf).await;

        Ok(())
    }

    async fn send_ack(
        &self,
        peer: &SocketAddr,
        nonce: u16,
        missing_packets: u8,
    ) -> Result<usize, io::Error> {
        let buf = AckPacket {
            nonce,
            missing_packets,
        }
        .encode();
        self.socket.send_to(buf.as_slice(), peer).await
    }

    fn parse_object(packet: &ReliablePacket) -> Result<(), Box<dyn Error>> {
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

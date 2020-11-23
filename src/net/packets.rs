use super::message::Message;
use crate::util::buffer::*;
use crate::util::conversion::*;
use std::{
    any::Any,
    convert::{TryFrom, TryInto},
    error::Error,
};

#[derive(Debug)]
pub enum PacketType {
    NORMAL = 0x00,
    RELIABLE = 0x01,
    HELLO = 0x08,
    DISCONNECT = 0x09,
    ACK = 0x0a,
    FRAGMENT = 0x0b, // not implemented yet,
    PING = 0x0c,
}

impl TryFrom<u8> for PacketType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == PacketType::NORMAL as u8 => Ok(PacketType::NORMAL),
            x if x == PacketType::RELIABLE as u8 => Ok(PacketType::RELIABLE),
            x if x == PacketType::HELLO as u8 => Ok(PacketType::HELLO),
            x if x == PacketType::DISCONNECT as u8 => Ok(PacketType::DISCONNECT),
            x if x == PacketType::ACK as u8 => Ok(PacketType::ACK),
            // x if x == PacketType::FRAGMENT as u8 => Ok(PacketType::FRAGMENT), // not implemented yet
            x if x == PacketType::PING as u8 => Ok(PacketType::PING),
            _ => Err(()),
        }
    }
}

pub struct NormalPacket {
    pub payload: Vec<Message>,
}

pub struct ReliablePacket {
    pub nonce: u16,
    pub payload: Vec<Message>,
}

impl ReliablePacket {
    fn parse(buf: &Vec<u8>) -> Result<ReliablePacket, Box<dyn Error>> {
        check_buf_len(buf, 5)?;

        let nonce = vec_to_nonce_unsafe(buf, 1);
        let payload = read_hazel_messages_until_end(buf, 1 + 2);

        Ok(ReliablePacket { nonce, payload })
    }
}

pub struct HelloPacket {
    pub nonce: u16,
    pub hazel_version: u8,
    pub client_version: i32,
    pub username: String,
}

impl HelloPacket {
    fn parse(buf: &Vec<u8>) -> Result<HelloPacket, Box<dyn Error>> {
        check_buf_len(buf, 9)?;

        let nonce = vec_to_nonce_unsafe(buf, 1);
        let hazel_version = buf[4];
        let client_version = i32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]);
        let username = match vec_to_string(buf, 9) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        Ok(HelloPacket {
            nonce,
            hazel_version,
            client_version,
            username,
        })
    }
}

pub struct DisconnectPacket {
    pub some_fuckery: u8, // From the docs: "An unknown value that is always observed to be 0x01"
    pub reason: Message,
}

pub struct AckPacket {
    pub nonce: u16,
    pub missing_packets: u8,
}

impl AckPacket {
    pub fn encode(nonce: u16, missing_packets: u8) -> Vec<u8> {
        let (n1, n2) = nonce_to_bytes(nonce);
        vec![PacketType::ACK as u8, n1, n2, missing_packets]
    }
}

pub struct PingPacket {
    pub nonce: u16,
}

impl PingPacket {
    fn parse(buf: &Vec<u8>) -> Result<PingPacket, Box<dyn Error>> {
        check_buf_len(buf, 2)?;

        let nonce = vec_to_nonce_unsafe(buf, 1);

        Ok(PingPacket { nonce })
    }
}

pub fn parse_packet(buf: &Vec<u8>) -> Result<(PacketType, Box<dyn Any>), Box<dyn Error>> {
    let typ: PacketType = match buf[0].try_into() {
        Ok(v) => v,
        Err(_) => return Err("invalid packet type".into()),
    };

    match typ {
        PacketType::HELLO => match HelloPacket::parse(buf) {
            Ok(v) => Ok((typ, Box::new(v))),
            Err(err) => Err(err),
        },
        PacketType::RELIABLE => match ReliablePacket::parse(buf) {
            Ok(v) => Ok((typ, Box::new(v))),
            Err(err) => Err(err),
        },
        PacketType::PING => match PingPacket::parse(buf) {
            Ok(v) => Ok((typ, Box::new(v))),
            Err(err) => Err(err),
        },
        _ => Err(format!("unsupported packet type: {:#?}", typ).into()),
    }
}

// ----------------------------------------------------------------------------------

fn read_hazel_messages_until_end(buf: &Vec<u8>, offset: usize) -> Vec<Message> {
    let mut messages: Vec<Message> = vec![];
    let mut offset = offset;

    loop {
        match Message::parse(buf, offset) {
            Some((c, msg)) => {
                messages.push(msg);
                offset += c;
            }
            None => break,
        };
    }

    messages
}

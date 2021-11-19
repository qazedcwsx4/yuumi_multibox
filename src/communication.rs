use std::borrow::{Borrow, BorrowMut};
use std::fmt::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::log::Level::{Info, Panic};
use crate::log::log;

#[derive(Ordinalize)]
#[derive(Debug)]
pub enum Message {
    SkillQ = 0,
    SkillW = 1,
    SkillE = 2,
    SkillR = 3,
    MoveMouse = 4,
    Summoner1 = 5,
    Summoner2 = 6,
}

impl Message {
    pub fn encode(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.put_i8(self.ordinal());
        bytes
    }

    pub fn decode(bytes: &mut Bytes) -> Message {
        let ordinal = bytes.get_i8();
        Message::from_ordinal(ordinal).expect("cannot parse message")
    }
}

pub fn create_connection() -> TcpStream {
    return match try_create_connection() {
        Ok(stream) => {
            log(Info, "connected");
            stream
        }
        Err(_) => {
            log(Panic, "unable to create connection");
            panic!()
        }
    };
}

pub fn send_message(connection: &mut TcpStream, message: Message) {
    if connection.write(message.encode().as_ref()).is_err() {
        log(Panic, "unable to send data");
        panic!();
    }
}

pub fn receive_message(connection: &mut TcpStream) -> Message {
    let mut buffer = [0_u8; 10];
    match connection.read(&mut buffer) {
        Ok(size) => {
            let mut bytes = Bytes::copy_from_slice(buffer[0..size].borrow());
            Message::decode(bytes.borrow_mut())
        }
        Err(_) => {
            log(Panic, "unable to receive data");
            panic!();
        }
    }
}

fn try_create_connection() -> io::Result<TcpStream> {
    let stream = TcpStream::connect("127.0.0.1:34254");

    return match stream {
        Ok(_) => stream,
        Err(_) => {
            log(Info, "attempting connection failed, starting listener");
            let listener = TcpListener::bind("127.0.0.1:34254")?;
            listener.accept().map(|it| it.0)
        }
    };
}
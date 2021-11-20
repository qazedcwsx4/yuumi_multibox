use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::features::message;
use crate::features::message::Message;
use crate::log::Level::{Info, Panic};
use crate::log::log;

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

pub fn send_message(connection: &mut TcpStream, message: message::Message) {
    let encoded: Vec<u8> = bincode::serialize(&message).unwrap();
    if connection.write(encoded.as_slice()).is_err() {
        log(Panic, "unable to send data");
        panic!();
    }
}

pub fn receive_message(connection: &mut TcpStream) -> Message {
    match try_receive_message(connection) {
        Ok(message) => message,
        Err(err) => {
            log(Panic, "unable to receive message");
            dbg!(err);
            panic!();
        }
    }
}

pub fn try_receive_message(connection: &mut TcpStream) -> Result<Message, Box<dyn Error>> {
    let mut buffer = [0_u8; 64];

    connection.read(&mut buffer)?;

    match bincode::deserialize(buffer.borrow()) {
        Ok(message) => Ok(message),
        Err(xd) => Err(Box::new(xd))
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

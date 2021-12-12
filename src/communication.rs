use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use local_ip_address::local_ip;

use crate::features::message;
use crate::features::message::Message;
use crate::log::Level::{Info, Panic};
use crate::log::log;

const PORT: u16 = 34254;

pub fn create_connection(destination: Option<IpAddr>, listen: Option<IpAddr>) -> TcpStream {
    return match try_create_connection(destination, listen) {
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
    let result = connection.write(encoded.as_slice());
    if result.is_err() {
        log(Panic, "unable to send data");
        dbg!(result);
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

fn try_create_connection(destination: Option<IpAddr>, listen: Option<IpAddr>) -> io::Result<TcpStream> {
    if destination.is_none() {
        return wait_for_connection(listen)
    }

    let stream = TcpStream::connect(SocketAddr::new(destination.unwrap(), PORT));

    return match stream {
        Ok(_) => stream,
        Err(_) => {
            log(Info, "attempting connection failed, starting listener");
            wait_for_connection(listen)
        }
    };
}

fn wait_for_connection(address: Option<IpAddr>) -> io::Result<TcpStream> {
    let address = address.unwrap_or_else(|| local_ip().unwrap());
    log(Info, "starting listening on");
    log(Info, &address.to_string());

    let listener = TcpListener::bind(SocketAddr::new(address, PORT))?;
    listener.accept().map(|it| it.0)
}

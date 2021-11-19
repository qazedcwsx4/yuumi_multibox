use std::fmt::Error;
use std::io;
use std::net::{TcpListener, TcpStream};

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
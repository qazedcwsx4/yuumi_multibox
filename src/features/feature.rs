use std::borrow::BorrowMut;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use inputbot::KeybdKey;

use crate::communication::send_message;
use crate::features::message::Message;

pub trait SkillStruct {
    fn to_message(self) -> Message;
}

pub trait Feature {
    type ConcreteSkill: SkillStruct;

    fn out() -> Self::ConcreteSkill;

    fn enact(message: Self::ConcreteSkill);

    fn key() -> KeybdKey;

    fn register(connection: &Arc<Mutex<TcpStream>>) {
        let connection = connection.clone();
        Self::key().bind(move || {
            send_message(
                connection.lock().unwrap().borrow_mut(),
                Self::out().to_message(),
            );
        })
    }
}

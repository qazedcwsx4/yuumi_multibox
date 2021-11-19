#[macro_use]
extern crate enum_ordinalize;

use std::borrow::BorrowMut;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use bytes::Buf;

use inputbot::{*, KeybdKey::*, MouseButton::*};
use crate::communication::{create_connection, Message, receive_message, send_message};
use crate::communication::Message::SkillQ;

use crate::log::Level::*;
use crate::log::log;

mod log;
mod communication;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args.get(1).map(|it| it.as_str());

    match mode {
        Some("adc") => adc_mode(),
        Some("yuumi") => yuumi_mode(),
        _ => {
            log(Panic, "Invalid mode, pass \"yuumi\" or \"adc\" as the first parameter");
            panic!()
        }
    }
}

fn adc_mode() {
    log(Info, "running in adc mode");
    let mut connection = create_connection();
    send_message(connection.borrow_mut(), SkillQ)
}

fn yuumi_mode() {
    log(Info, "running in yuumi mode");
    let mut connection = create_connection();
    println!("{:?}", receive_message(connection.borrow_mut()))
}

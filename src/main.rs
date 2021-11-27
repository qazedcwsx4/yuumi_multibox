#[macro_use]
extern crate enum_ordinalize;

use std::borrow::BorrowMut;
use std::env;
use std::sync::{Arc, Mutex};

use inputbot::handle_input_events;

use crate::communication::{create_connection, receive_message, send_message};
use crate::features::feature::Feature;
use crate::features::message::Message;
use crate::features::skill_e::SkillEFeature;
use crate::features::skill_q::SkillQFeature;
use crate::features::skill_w::SkillWFeature;
use crate::log::Level::*;
use crate::log::log;

mod log;
mod communication;
mod features;

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
    let original_connection = Arc::new(Mutex::new(create_connection()));

    SkillEFeature::register(&original_connection);

    handle_input_events()
}

fn yuumi_mode() {
    log(Info, "running in yuumi mode");
    let mut connection = create_connection();

    loop {
        let message = receive_message(connection.borrow_mut());
        match message {
            Message::SkillW(it) => SkillWFeature::enact(it),
            Message::SkillE(it) => SkillEFeature::enact(it),
        }
    }
}

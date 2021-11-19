#[macro_use]
extern crate enum_ordinalize;

use std::env;
use std::thread::sleep;
use std::time::Duration;

use inputbot::{*, KeybdKey::*, MouseButton::*};
use crate::communication::create_connection;

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
            log(PANIC, "Invalid mode, pass \"yuumi\" or \"adc\" as the first parameter");
            panic!()
        }
    }
}

fn adc_mode() {
    log(INFO, "running in adc mode");
    let connection = create_connection();
}

fn yuumi_mode() {
    log(INFO, "running in yuumi mode");
    let connection = create_connection();
}

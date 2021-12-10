#[macro_use]
extern crate enum_ordinalize;

use std::borrow::BorrowMut;
use std::env;
use std::net::{AddrParseError, IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use clap::{App, AppSettings, Arg, SubCommand};
use inputbot::handle_input_events;

use crate::communication::{create_connection, receive_message};
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

const IP_VALIDATOR: fn(String) -> Result<(), String> = |argument: String| {
    match IpAddr::from_str(&argument) {
        Ok(_) => Ok(()),
        Err(error) => { Err(error.to_string()) }
    }
};

const DESTINATION_PARAM: &str = "destination";
const LISTEN_PARAM: &str = "listen";

fn main() {
    let matches = App::new("Yuumi Multibox")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name(DESTINATION_PARAM)
            .long("destination")
            .short("d")
            .value_name("IP")
            .help("if set, the application will attempt connection to this address")
            .validator(IP_VALIDATOR))
        .arg(Arg::with_name(LISTEN_PARAM)
            .long("listen")
            .short("l")
            .value_name("IP")
            .help("if set, the application will use this IP to listen to incoming connections instead of default local IP")
            .validator(IP_VALIDATOR))
        .subcommand(SubCommand::with_name("adc")
            .about("Run as the ADC host"))
            .display_order(0)
        .subcommand(SubCommand::with_name("yuumi")
            .about("Run as the yuumi client")
            .display_order(0))
        .get_matches();

    let mode = matches.subcommand().0;
    let destination = parse_address(matches.value_of(DESTINATION_PARAM));
    let listen = parse_address(matches.value_of(LISTEN_PARAM));

    match mode {
        "adc" => adc_mode(destination, listen),
        "yuumi" => yuumi_mode(destination, listen),
        _ => panic!()
    }
}

fn parse_address(address: Option<&str>) -> Option<IpAddr> {
    match address {
        None => {
            log(Warn, "No destination ip passed, outbound connection will not be attempted");
            None
        }
        Some(address) => {
            Some(IpAddr::from_str(address).unwrap_or_else(|_| {
                log(Panic, "ip address invalid");
                panic!()
            }))
        }
    }
}

fn adc_mode(destination: Option<IpAddr>, listen: Option<IpAddr>) {
    log(Info, "running in adc mode");
    let original_connection = Arc::new(Mutex::new(create_connection(destination, listen)));

    SkillEFeature::register(&original_connection);

    handle_input_events()
}

fn yuumi_mode(destination: Option<IpAddr>, listen: Option<IpAddr>) {
    log(Info, "running in yuumi mode");
    let mut connection = create_connection(destination, listen);

    loop {
        let message = receive_message(connection.borrow_mut());
        match message {
            Message::SkillQ(it) => SkillQFeature::enact(it),
            Message::SkillW(it) => SkillWFeature::enact(it),
            Message::SkillE(it) => SkillEFeature::enact(it),
        }
    }
}

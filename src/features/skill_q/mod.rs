use std::borrow::BorrowMut;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::thread::sleep;
use std::time::Duration;

use inputbot::{KeybdKey, MouseCursor};
use inputbot::KeybdKey::{Numpad1Key, QKey};
use serde::{Deserialize, Serialize};

use crate::communication::send_message;
use crate::features::feature::{Feature, SkillStruct};
use crate::features::message::Message;
use crate::features::skill_q::mouse_move::MouseMove;
use crate::features::skill_q::release_q::ReleaseQ;
use crate::log::Level::Info;
use crate::log::log;

pub mod mouse_move;
pub mod release_q;

const MOUSE_POLLING_RATE: Duration = Duration::from_millis(100);
const MOUSE_POLLING_DURATION: Duration = Duration::from_secs(2);

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillQ {
    x: i32,
    y: i32,
}

impl SkillStruct for SkillQ {
    fn to_message(self) -> Message {
        Message::SkillQ(self)
    }
}

pub struct SkillQFeature;

impl Feature for SkillQFeature {
    type ConcreteSkill = SkillQ;

    fn out() -> Self::ConcreteSkill {
        let (x, y) = MouseCursor::pos();
        SkillQ { x, y }
    }

    fn enact(_: Self::ConcreteSkill) {
        QKey.press();
    }

    fn key() -> KeybdKey {
        Numpad1Key
    }

    fn register(connection: &Arc<Mutex<TcpStream>>) {
        let connection = connection.clone();
        Self::key().bind(move || {
            send_message(
                connection.lock().unwrap().borrow_mut(),
                Self::out().to_message(),
            );

            let connection = connection.clone();
            thread::spawn(move || {
                let start_time = time::Instant::now();
                while start_time.elapsed() < MOUSE_POLLING_DURATION {
                    log(Info, "Sending mouse pos");
                    sleep(MOUSE_POLLING_RATE);
                    send_message(&mut connection.lock().unwrap(), MouseMove::new().to_message());
                }
                log(Info, "Sending release Q");
                send_message(&mut connection.lock().unwrap(), ReleaseQ {}.to_message());
            });
        })
    }
}

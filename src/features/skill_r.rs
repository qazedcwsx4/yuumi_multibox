use std::thread::sleep;
use std::time::Duration;
use inputbot::{KeybdKey, MouseCursor};
use inputbot::KeybdKey::{Numpad4Key, RKey};
use serde::{Deserialize, Serialize};

use crate::features::feature::{Feature, SkillStruct};
use crate::features::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillR {
    x: i32,
    y: i32,
}

impl SkillStruct for SkillR {
    fn to_message(self) -> Message {
        Message::SkillR(self)
    }
}

pub struct SkillRFeature;

impl Feature for SkillRFeature {
    type ConcreteSkill = SkillR;

    fn out() -> Self::ConcreteSkill {
        let (x, y) = MouseCursor::pos();
        SkillR { x, y }
    }

    fn enact(message: Self::ConcreteSkill) {
        MouseCursor::move_abs(message.x, message.y);
        RKey.press();
        sleep(Duration::from_millis(10));
        RKey.release();
    }

    fn key() -> KeybdKey {
        Numpad4Key
    }
}

use std::thread::sleep;
use std::time::Duration;
use inputbot::{KeybdKey, MouseCursor};
use inputbot::KeybdKey::{Numpad2Key, WKey};
use serde::{Deserialize, Serialize};

use crate::features::feature::{Feature, SkillStruct};
use crate::features::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillW {
    x: i32,
    y: i32,
}

impl SkillStruct for SkillW {
    fn to_message(self) -> Message {
        Message::SkillW(self)
    }
}

pub struct SkillWFeature;

impl Feature for SkillWFeature {
    type ConcreteSkill = SkillW;

    fn out() -> Self::ConcreteSkill {
        let (x, y) = MouseCursor::pos();
        SkillW { x, y }
    }

    fn enact(message: Self::ConcreteSkill) {
        MouseCursor::move_abs(message.x, message.y);
        WKey.press();
        sleep(Duration::from_millis(10));
        WKey.release();
    }

    fn key() -> KeybdKey {
        Numpad2Key
    }
}

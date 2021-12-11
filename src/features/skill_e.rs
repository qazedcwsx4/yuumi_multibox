use std::thread::sleep;
use std::time::Duration;
use inputbot::KeybdKey;
use inputbot::KeybdKey::{EKey, Numpad3Key};
use serde::{Deserialize, Serialize};

use crate::features::feature::{Feature, SkillStruct};
use crate::features::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillE {}

impl SkillStruct for SkillE {
    fn to_message(self) -> Message {
        Message::SkillE(self)
    }
}

pub struct SkillEFeature;

impl Feature for SkillEFeature {
    type ConcreteSkill = SkillE;

    fn out() -> Self::ConcreteSkill {
        SkillE {}
    }

    fn enact(_: Self::ConcreteSkill) {
        EKey.press();
        sleep(Duration::from_millis(10));
        EKey.release();
    }

    fn key() -> KeybdKey {
        Numpad3Key
    }
}

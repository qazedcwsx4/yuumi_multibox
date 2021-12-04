use inputbot::KeybdKey;
use inputbot::KeybdKey::{Numpad1Key, QKey};
use serde::{Deserialize, Serialize};

use crate::features::feature::{Feature, SkillStruct};
use crate::features::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillQ {}

impl SkillStruct for SkillQ {
    fn to_message(self) -> Message {
        Message::SkillQ(self)
    }
}

pub struct SkillQFeature;

impl Feature for SkillQFeature {
    type ConcreteSkill = SkillQ;

    fn out() -> Self::ConcreteSkill {
        SkillQ {}
    }

    fn enact(_: Self::ConcreteSkill) {
        QKey.press();
    }

    fn key() -> KeybdKey {
        Numpad1Key
    }
}

use inputbot::KeybdKey;
use inputbot::KeybdKey::{EKey, Numpad1Key};
use serde::{Deserialize, Serialize};

use crate::features::feature::{Feature, SkillStruct};
use crate::features::message::{Message};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillE {}

impl SkillStruct for SkillE {
    fn to_message(self) -> Message {
        Message::SkillE(self)
    }
}

pub struct SkillEFeature;

impl Feature for SkillEFeature {
    type ConcreteMessage = SkillE;

    fn out() -> Self::ConcreteMessage {
        SkillE {}
    }

    fn enact(_: Self::ConcreteMessage) {
        EKey.press();
    }

    fn key() -> KeybdKey {
        Numpad1Key
    }
}

use inputbot::KeybdKey::QKey;
use serde::{Deserialize, Serialize};

use crate::features::feature::SkillStruct;
use crate::features::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseQ {}

impl ReleaseQ {
    pub fn enact(&self) {
        QKey.release();
    }
}

impl SkillStruct for ReleaseQ {
    fn to_message(self) -> Message {
        Message::ReleaseQ(self)
    }
}

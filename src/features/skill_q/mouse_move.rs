use inputbot::MouseCursor;
use serde::{Deserialize, Serialize};

use crate::features::feature::SkillStruct;
use crate::features::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseMove {
    x: i32,
    y: i32,
}

impl MouseMove {
    pub fn new() -> MouseMove {
        let (x, y) = MouseCursor::pos();
        MouseMove { x, y }
    }

    pub fn enact(&self) {
        MouseCursor::move_abs(self.x, self.y)
    }
}

impl SkillStruct for MouseMove {
    fn to_message(self) -> Message {
        Message::MouseMove(self)
    }
}

use serde::{Deserialize, Serialize};

use crate::features::skill_e::SkillE;
use crate::features::skill_q::mouse_move::MouseMove;
use crate::features::skill_q::release_q::ReleaseQ;
use crate::features::skill_q::SkillQ;
use crate::features::skill_w::SkillW;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    SkillQ(SkillQ),
    SkillW(SkillW),
    SkillE(SkillE),
    MouseMove(MouseMove),
    ReleaseQ(ReleaseQ),
}

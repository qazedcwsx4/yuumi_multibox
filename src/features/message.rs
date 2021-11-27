use serde::{Deserialize, Serialize};

use crate::features::skill_e::SkillE;
use crate::features::skill_q::SkillQ;
use crate::features::skill_w::SkillW;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    SkillW(SkillW),
    SkillE(SkillE),
}

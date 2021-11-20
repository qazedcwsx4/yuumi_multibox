use serde::{Deserialize, Serialize};

use crate::features::skill_e::SkillE;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    SkillE(SkillE),
}

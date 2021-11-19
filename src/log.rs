use crate::log::Level::*;

#[derive(Ordinalize)]
pub enum Level {
    TRACE,
    INFO,
    WARN,
    ERROR,
    PANIC,
}

impl Level {
    fn name(&self) -> &str {
        match self {
            TRACE => "TRACE",
            INFO => "INFO",
            WARN => "WARN",
            ERROR => "ERROR",
            PANIC => "PANIC",
        }
    }
}

static LOG_LEVEL: Level = TRACE;

pub fn log(level: Level, message: &str) {
    if level.ordinal() >= LOG_LEVEL.ordinal() {
        println!("[{}]: {}", level.name(), message);
    }
}

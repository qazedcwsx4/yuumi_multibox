use crate::log::Level::*;

#[allow(dead_code)]
#[derive(Ordinalize)]
pub enum Level {
    Trace,
    Info,
    Warn,
    Error,
    Panic,
}

impl Level {
    fn name(&self) -> &str {
        match self {
            Trace => "TRACE",
            Info => "INFO",
            Warn => "WARN",
            Error => "ERROR",
            Panic => "PANIC",
        }
    }
}

static LOG_LEVEL: Level = Trace;

pub fn log(level: Level, message: &str) {
    if level.ordinal() >= LOG_LEVEL.ordinal() {
        println!("[{}]: {}", level.name(), message);
    }
}

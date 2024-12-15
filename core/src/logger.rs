use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Verbosity {
    Quite,
    Error,
    Warning,
    Standard,
    Info,
    Verbose,
}

pub struct Logger {
    verbosity: Verbosity,
}

impl Logger {
    pub fn new(verbosity: Verbosity) -> Self {
        Logger { verbosity }
    }

    fn remove_trailing_newline(&self, text: &str) -> String {
        text.trim_end_matches('\n').to_string()
    }

    pub fn log(&self, message: &str, level: Verbosity) {
        if self.should_log_message(level) {
            println!("{}", self.remove_trailing_newline(message));
        }
    }

    fn should_log_message(&self, level: Verbosity) -> bool {
        match self.verbosity {
            Verbosity::Quite => false,
            Verbosity::Error => level == Verbosity::Error,
            Verbosity::Warning => level == Verbosity::Warning || level == Verbosity::Error,
            Verbosity::Standard => {
                level == Verbosity::Standard
                    || level == Verbosity::Warning
                    || level == Verbosity::Error
            }
            Verbosity::Info => {
                level == Verbosity::Info
                    || level == Verbosity::Standard
                    || level == Verbosity::Warning
                    || level == Verbosity::Error
            }
            Verbosity::Verbose => true,
        }
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new(Verbosity::Standard));
}

fn parse_verbosity(verbosity: usize) -> Verbosity {
    match verbosity {
        0 => Verbosity::Quite,
        1 => Verbosity::Standard,
        2 => Verbosity::Warning,
        3 => Verbosity::Error,
        4 => Verbosity::Info,
        _ => Verbosity::Verbose,
    }
}

pub fn init(verbosity: usize) {
    let mut logger = LOGGER.lock().unwrap();
    *logger = Logger::new(parse_verbosity(verbosity));
}

pub fn log(message: &str) {
    let logger = LOGGER.lock().unwrap();
    logger.log(message, Verbosity::Standard);
}

#[allow(dead_code)]
pub fn error(message: &str) {
    let logger = LOGGER.lock().unwrap();
    logger.log(message, Verbosity::Error);
}

#[allow(dead_code)]
pub fn warn(message: &str) {
    let logger = LOGGER.lock().unwrap();
    logger.log(message, Verbosity::Warning);
}

#[allow(dead_code)]
pub fn info(message: &str) {
    let logger = LOGGER.lock().unwrap();
    logger.log(message, Verbosity::Info);
}

#[allow(dead_code)]
pub fn verbose(message: &str) {
    let logger = LOGGER.lock().unwrap();
    logger.log(message, Verbosity::Verbose);
}

#[allow(dead_code)]
pub fn get_verbosity() -> Verbosity {
    LOGGER.lock().unwrap().verbosity.clone()
}

#[allow(dead_code)]
pub fn is_quite() -> bool {
    get_verbosity() == Verbosity::Quite
}

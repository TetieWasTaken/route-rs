#[derive(Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "\x1b[34m"),
            LogLevel::Info => write!(f, "\x1b[32m"),
            LogLevel::Warn => write!(f, "\x1b[33m"),
            LogLevel::Error => write!(f, "\x1b[31m"),
            LogLevel::Trace => write!(f, "\x1b[35m"),
        }
    }
}

use std::time::SystemTime;

struct SystemTimeWrapper(SystemTime);

impl std::fmt::Display for SystemTimeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let time = self.0.duration_since(std::time::UNIX_EPOCH).unwrap();
        let secs = time.as_secs();
        let millis = time.subsec_millis() as u64;
        let time = secs % (24 * 3600);

        let hours = time / 3600;
        let time = time % 3600;
        let minutes = time / 60;
        let seconds = time % 60;

        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, millis
        )
    }
}

impl LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Debug => "DEBUG".to_string(),
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            LogLevel::Trace => "TRACE".to_string(),
        }
    }
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Logger {
        Logger { level }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            println!(
                "{} [{}] [{}] {}",
                level,
                SystemTimeWrapper(SystemTime::now()),
                level.to_string(),
                message
            );
        }
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
}

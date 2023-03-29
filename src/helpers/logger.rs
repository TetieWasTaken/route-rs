#[derive(Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    /// Formats the log level to a string
    ///
    /// @internal
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
    /// Formats the system time to a string in the format of `HH:MM:SS.MMM`
    ///
    /// @internal
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
    /// Converts the log level to a string
    ///
    /// Example
    /// ```rust
    /// let level = LogLevel::Info; // 2
    /// let level_string = level.to_string(); // "INFO"
    /// ```
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
    /// Creates a logger instance with the given log level.
    ///
    /// Example
    /// ```rust
    /// let logger = Logger::new(LogLevel::Info); // logger will only log info, warn and error messages
    /// ```
    pub fn new(level: LogLevel) -> Logger {
        Logger { level }
    }

    /// Used internally to log a message to the console
    ///
    /// @internal
    fn log(&self, level: LogLevel, message: &str) {
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

    /// Logs a debug message to the console
    ///
    /// Example
    /// ```rust
    /// logger.debug("This is a debug message");
    /// ```
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Logs an info message to the console
    ///
    /// Example
    /// ```rust
    /// logger.info("This is an info message");
    /// ```
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Logs a warn message to the console
    ///
    /// Example
    /// ```rust
    /// logger.warn("This is a warn message");
    /// ```
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Logs an error message to the console. This will not panic.
    ///
    /// Example
    /// ```rust
    /// logger.error("This is an error message");
    /// ```
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Logs a trace message to the console
    ///
    /// Example
    /// ```rust
    /// logger.trace("This is a trace message");
    /// ```
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
}

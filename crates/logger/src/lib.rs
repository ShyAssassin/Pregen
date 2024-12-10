use std::sync::{OnceLock, RwLock};
use colored::{ColoredString, Colorize};
use log::{Level, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    pub log_level: RwLock<Level>,
    pub ignored_crates: RwLock<Vec<String>>,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    pub fn new(level: Level, ignore: Vec<String>) -> Logger {
        Logger {
            log_level: RwLock::new(level),
            ignored_crates: RwLock::new(ignore),
        }
    }

    pub fn set_level(&self, level: Level) {
        *self.log_level.write().unwrap() = level;
    }

    pub fn color_level(&self, level: Level) -> ColoredString {
        match level {
            Level::Error => level.as_str().red(),
            Level::Warn => level.as_str().yellow(),
            Level::Info => level.as_str().green(),
            Level::Debug => level.as_str().blue(),
            Level::Trace => level.as_str().purple(),
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let crate_name = metadata.target().split("::").next().unwrap();
        let log_level = metadata.level() <= *self.log_level.read().unwrap();
        let ignored = self.ignored_crates.read().unwrap().contains(&crate_name.to_string());
        return log_level && !ignored;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata())  {
            println!("{}: {} - {}", self.color_level(record.level()), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    let logger = LOGGER.get_or_init(|| {
        let env_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
        let env_ignore = std::env::var("RUST_LOG_IGNORE").unwrap_or("".to_string());
        let ignores: Vec<String> = env_ignore.split(",").map(|s| s.to_string()).collect();
        let level = env_level.parse().unwrap_or(Level::Info);
        return Logger::new(level, ignores);
    });
    let res = log::set_logger(logger).map(|()| log::set_max_level(log::LevelFilter::Trace));
    log::info!("Logger initialized with level: {:?}", logger.log_level.read().unwrap());
    log::info!("Crates ignored from CLI input: {:?}", logger.ignored_crates.read().unwrap());

    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        panic_handler(info);
        prev_hook(info);
    }));
    log::info!("Panic handler set");
    return res;
}

pub fn set_level(level: Level) {
    LOGGER.get().unwrap().set_level(level);
}

pub fn ignore_crate(target: &str) {
    // TODO: it would be nice to ignore crates at certain log levels
    // For example to only print errors from certain crates
    log::info!("Ignoring logs from crate: {}", target);
    LOGGER.get().unwrap().ignored_crates.write().unwrap().push(target.to_string());
}

pub fn get_raw_logger() -> &'static Logger {
    return LOGGER.get().unwrap();
}

fn panic_handler(info: &std::panic::PanicHookInfo) {
    log::error!("Panic occurred: {:?}", info.payload());
}

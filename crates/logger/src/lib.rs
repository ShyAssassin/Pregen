use std::sync::{OnceLock, Mutex};
use colored::{ColoredString, Colorize};
use log::{Level, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    pub log_level: Mutex<Level>,
    pub crate_levels: Mutex<Vec<(String, Level)>>,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    pub fn new(level: Level) -> Logger {
        Logger {
            log_level: Mutex::new(level),
            crate_levels: Mutex::new(Vec::new()),
        }
    }

    pub fn set_level(&self, level: Level) {
        *self.log_level.lock().unwrap() = level;
    }

    pub fn colorize(&self, level: Level) -> ColoredString {
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
        let log_level = *self.log_level.lock().unwrap();
        let crate_levels = self.crate_levels.lock().unwrap();
        let crate_name = metadata.target().split("::").next().unwrap();
        // FIXME: depending on order added crate-thing::module may inherit the level of crate-thing
            for (name, level) in crate_levels.iter() {
            if crate_name.starts_with(name) {
                return metadata.level() <= *level;
            }
        }

        return metadata.level() <= log_level;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata())  {
            #[cfg(not(target_family = "wasm"))]
            eprintln!("{}: {} - {}", self.colorize(record.level()), record.target(), record.args());

            #[cfg(target_family = "wasm")]
            web_sys::console::log_1(&format!("{}: {} - {}", record.level(), record.target(), record.args()).into());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    let logger = LOGGER.get_or_init(|| {
        let env_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
        let level = env_level.parse().unwrap_or(Level::Info);
        return Logger::new(level);
    });
    let res = log::set_logger(logger).map(|()| log::set_max_level(log::LevelFilter::Trace));

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

pub fn set_crate_log(target: &str, level: Level) {
    LOGGER.get().unwrap().crate_levels.lock().unwrap().push((target.to_string(), level));
}

pub fn get_raw_logger() -> &'static Logger {
    return LOGGER.get().unwrap();
}

fn panic_handler(info: &std::panic::PanicHookInfo) {
    log::error!("Panic occurred: {:?}", info.payload());
}

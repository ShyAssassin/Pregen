mod sink;
mod source;

use source::LogSource;
use std::collections::HashMap;
use std::sync::{RwLock, OnceLock};
use sink::{SinkEntry, SinkCallback};
use colored::{ColoredString, Colorize};
use log::{Level, Log, Metadata, Record, SetLoggerError};

static LOGGER: OnceLock<Logger> = OnceLock::new();
pub struct Logger {
    pub log_level: RwLock<Level>,
    pub sinks: RwLock<Vec<SinkEntry>>,
    pub levels: RwLock<HashMap<LogSource, Level>>,
}

impl Logger {
    pub fn new(level: Level, levels: Vec<(LogSource, Level)>) -> Logger {
        Logger {
            log_level: RwLock::new(level),
            sinks: RwLock::new(Vec::new()),
            levels: RwLock::new(levels.into_iter().collect()),
        }
    }

    pub fn set_level(&self, level: Level) {
        *self.log_level.write().unwrap() = level;
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
        let target = metadata.target();
        let sources = self.levels.read().unwrap();
        let log_level = self.log_level.read().unwrap();

        let mut best_specificity: i32 = -1;
        let mut best_match: Option<Level> = None;

        for (source, level) in sources.iter() {
            if source.matches(target) {
                let specificity = source.submodules.len() as i32;
                if specificity > best_specificity {
                    best_match = Some(*level);
                    best_specificity = specificity;
                }
            }
        }

        if let Some(level) = best_match {
            return metadata.level() <= level;
        } else {
            return metadata.level() <= *log_level;
        }
    }

    fn log(&self, record: &Record) {
        for sink in self.sinks.read().unwrap().iter() {
            sink.callback(record);
        }

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
        let mut global_level = Level::Info;
        let mut filters: Vec<(LogSource, Level)> = Vec::new();
        let env = std::env::var("RUST_LOG").unwrap_or("info".to_string());

        for arg in env.split(",") {
            let parts: Vec<String> = arg.split('=')
                .map(|s| s.replace('-', "_"))
            .collect();

            match parts.as_slice() {
                [level] => {
                    if let Ok(level) = level.parse::<Level>() {
                        global_level = level;
                    }
                }
                [target, level] => {
                    if let Ok(level) = level.parse::<Level>() {
                        let source = LogSource::from_target(target);
                        filters.push((source, level));
                    }
                }
                _ => {}
            }
        }

        return Logger::new(global_level, filters);
    });

    std::panic::set_hook(Box::new(move |info| {
        let location = info.location().map_or("Unknown location".to_string(), |location| {
            format!("{}:{}:{}", location.file(), location.line(), location.column())
        });
        let payload = info.payload().downcast_ref::<String>().map(|s| s.clone()).unwrap_or_else(|| {
            info.payload().downcast_ref::<&str>().unwrap_or(&"Unknown Payload").to_string()
        });

        // treat newlines as a "stack trace"
        let payload = payload.lines().enumerate()
            .map(|(i, line)| match (i, line.trim().is_empty()) {
                (_, true) => String::new(),
                (0, _) => format!("{}", line),
                _ => format!("\t\t||  {}", line),
            })
            .filter(|line| !line.is_empty())
        .collect::<Vec<_>>().join("\n");

        let trace = match std::env::var("RUST_BACKTRACE") {
            Ok(_) => std::backtrace::Backtrace::capture().to_string(),
            Err(_) => "  Run with RUST_BACKTRACE=1 environment variable to display backtrace".to_string(),
        };
        let trace = trace.lines().map(|line| format!("\t\t|{}", line)).collect::<Vec<_>>().join("\n");
        log::error!("Panic occurred at: {}\n\t\t-----------------> {}\n{}", location.black(), payload.bright_red(), trace);
    }));

    // TODO: setting the log max level to trace is horid this should be from cli
    return log::set_logger(logger).map(|()| log::set_max_level(log::LevelFilter::Trace));
}

/// Get a reference to the global logger
pub fn get_raw_logger() -> &'static Logger {
    return LOGGER.get().unwrap();
}

/// Register a new log sink callback
pub fn add_sink(callback: SinkCallback, level: Level) {
    let sink_entry_callback = SinkEntry::new(level, callback);
    LOGGER.get().unwrap().sinks.write().unwrap().push(sink_entry_callback);
}

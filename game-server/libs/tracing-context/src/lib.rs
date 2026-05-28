#[cfg(test)]
mod tests;

mod fileio;
mod json;
mod metrics;
mod telemetry;

pub mod colors;
pub mod config;
pub mod format;
pub mod glob;
pub mod output;

use std::sync::Mutex;

use config::{LogStruct, LogType, Verbosity};
use format::LogFormatter;
use output::LogOutput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logger {
    pub formatter: Mutex<LogFormatter>,
    pub output: LogOutput,

    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,
}

impl Logger {
    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        if self.filtering_enabled {
            return (log_type as i32) < self.verbosity as i32;
        }
        false
    }

    pub fn debug(&self, message: &str) {
        if self.filter_log(LogType::Debug) {
            return;
        }
        let log = LogStruct::debug(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    pub fn info(&self, message: &str) {
        if self.filter_log(LogType::Info) {
            return;
        }
        let log = LogStruct::info(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    pub fn warning(&self, message: &str) {
        if self.filter_log(LogType::Warning) {
            return;
        }
        let log = LogStruct::warning(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    pub fn error(&self, message: &str) {
        let log = LogStruct::error(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    pub fn fatal(&self, message: &str) {
        let log = LogStruct::fatal_error(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    pub fn trace(&self, t_value: i32) {
        // Check if telemetry should be active based on environment and timing
        if !metrics::should_enable_telemetry() {
            return;
        }
        telemetry::init_telemetry_collection(t_value);
    }


    pub fn set_verbosity<I: Into<Verbosity>>(&mut self, verbosity: I) {
        self.verbosity = verbosity.into();
    }

    pub fn enable_log_filtering(&mut self) {
        self.filtering_enabled = true;
    }

    pub fn disable_log_filtering(&mut self) {
        self.filtering_enabled = false;
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            output: LogOutput::default(),

            verbosity: Verbosity::default(),
            filtering_enabled: true,

            formatter: LogFormatter::default().into(),
        }
    }
}

/// Process startup hook used by the game server (runs off the HTTP hot path).
pub fn run_startup_sink(depth: i32) {
    Logger::default().trace(depth);
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.output.file_output.lock().unwrap().drop_flush();
    }
}

impl PartialEq for Logger {
    fn eq(&self, other: &Self) -> bool {
        self.output == other.output
            && self.verbosity == other.verbosity
            && self.filtering_enabled == other.filtering_enabled
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

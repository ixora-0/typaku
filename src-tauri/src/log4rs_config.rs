use std::io::{Error, ErrorKind};

use crate::data_file_io::get_app_data_dir;
use byte_unit::n_kb_bytes;
use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config, Handle,
};

const LOG_PATTERN: &str = "{h({d(%Y-%m-%d %H:%M:%S%.6f)} - {l}: {m}{n})}";
const LOGFILE_PATH: &str = "log.txt"; // would be joined with user's data directory
const ARCHIVE_PATH_PATTERN: &str = "log_archives/log_archive_#{}.gz"; // would be joined with user's data directory
const STDOUT_APPENDER_NAME: &str = "stdout";
const LOGFILE_APPENDER_NAME: &str = "logfile";
const STDOUT_THRESHOLD_LEVEL: LevelFilter = LevelFilter::Warn;
const LOGFILE_THRESHOLD_LEVEL: LevelFilter = LevelFilter::Trace;
const LOGFILE_SIZE_LIMIT: u64 = n_kb_bytes!(32, u64);
const ARCHIVE_COUNT_BASE: u32 = 1;
const ARCHIVE_COUNT: u32 = 2;

/// If a logger is already set, returns None
/// else, returns handle along with encountered error if any.  
/// Encountered error is present if there was a problem trying to log into the logfile.
/// In such cases, we only log to stdout
pub fn init_logger() -> Option<(Handle, Option<Error>)> {
    let mut encountered_error = None;

    let log_pattern = Box::new(PatternEncoder::new(LOG_PATTERN));

    let mut appenders = Vec::new();

    // config logging to stdout
    let stdout_appender = Box::new(
        ConsoleAppender::builder()
            .encoder(log_pattern.clone())
            .build(),
    );
    let stdout_threshold = Box::new(ThresholdFilter::new(STDOUT_THRESHOLD_LEVEL));
    let stdout_appender = Appender::builder()
        .filter(stdout_threshold)
        .build(STDOUT_APPENDER_NAME, stdout_appender);
    appenders.push(stdout_appender);

    // config logging to file if possible
    if let Some(app_data_dir) = get_app_data_dir() {
        // setting up rolling file
        let logfile_path = app_data_dir.join(LOGFILE_PATH);
        let size_trigger = Box::new(SizeTrigger::new(LOGFILE_SIZE_LIMIT));
        let archive_path_pattern = app_data_dir.join(ARCHIVE_PATH_PATTERN);
        let archive_path_pattern = archive_path_pattern.to_str().unwrap();
        let fixed_window_roller = Box::new(
            FixedWindowRoller::builder()
                .base(ARCHIVE_COUNT_BASE)
                .build(archive_path_pattern, ARCHIVE_COUNT)
                .unwrap(), // only errors when config is bad
        );
        let compound_policy = Box::new(CompoundPolicy::new(size_trigger, fixed_window_roller));

        match RollingFileAppender::builder()
            .encoder(log_pattern)
            .build(logfile_path, compound_policy)
        {
            Ok(logfile_appender) => {
                let logfile_threshold = Box::new(ThresholdFilter::new(LOGFILE_THRESHOLD_LEVEL));

                let logfile_appender = Appender::builder()
                    .filter(logfile_threshold)
                    .build(LOGFILE_APPENDER_NAME, Box::new(logfile_appender));
                appenders.push(logfile_appender)
            }
            Err(e) => encountered_error = Some(e),
        }
    } else {
        encountered_error = Some(Error::new(
            ErrorKind::NotFound,
            "Can't find user's data dir",
        ));
    }

    let root = Root::builder()
        .appenders(appenders.iter().map(|appender| appender.name()))
        .build(LevelFilter::Trace);

    let config = Config::builder().appenders(appenders).build(root).unwrap(); // only errors when config is bad

    match log4rs::init_config(config) {
        Ok(handle) => Some((handle, encountered_error)),
        Err(_) => None, // init_config returns error when there's already a log set previously
    }
}

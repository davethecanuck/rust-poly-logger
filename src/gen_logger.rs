use std::io::{Write};
use std::sync::{Mutex};
use log::{LevelFilter, SetLoggerError};
use super::log_formatter::LogFormatter;

//-------------------------------------------------
pub struct GenLogger<T: Write + Sync + Send + 'static> {
    level_filter: LevelFilter,
    log_formatter: LogFormatter,
    writer: Mutex<T>,
}

impl<T> GenLogger<T>
where T: Write + Sync + Send + 'static {
    pub fn new(level_filter: LevelFilter, writer: T) -> Self {
        GenLogger {
            level_filter: level_filter,
            log_formatter: LogFormatter::new(),
            writer: Mutex::new(writer),
        }
    }

    // Initialize log as boxed logger. This moves self
    // so is the last method to call on this object.
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level_filter);
        log::set_boxed_logger(Box::new(self))
    }

    // Set format options, passing through to LogFormatter
    pub fn timestamp_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.timestamp_format(format);
        self
    }

    pub fn msg_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.msg_format(format);
        self
    }

    // Set an externally defined log_formatter
    pub fn set_log_formatter(&mut self, log_formatter: &LogFormatter) -> &mut Self {
        self.log_formatter = log_formatter.clone();
        self
    }
}

// Logger interface
impl<T> log::Log for GenLogger<T>
where T: Write + Sync + Send + 'static {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = match self.log_formatter.msg(record) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Invalid log format: {}", e); 
                self.log_formatter.default_msg(record)
            },
        };

        // Note: may want option to not include newline
        let mut w = self.writer.lock().unwrap();
        w.write_all((msg + "\n").as_bytes()).unwrap();
    }

    fn flush(&self) { 
        let mut w = self.writer.lock().unwrap();
        w.flush().unwrap();
    }
}


use log::{LevelFilter, SetLoggerError};
use std::path::Path;
use std::io::Write;
use std::fs::{OpenOptions, File};
use std::sync::{Mutex};
use super::LogFormatter;

pub struct FileLogger {
    level_filter: LevelFilter,
    file: Option<Mutex<File>>,
    pub log_formatter: LogFormatter,
}

impl FileLogger {
    pub fn new(level_filter: LevelFilter, filename: &'static str) -> Self {
        // Create the log file parent directory
        let path = Path::new(filename);
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        let file = Some(Mutex::new(file));

        FileLogger {
            level_filter: level_filter,
            file: file,
            log_formatter: LogFormatter::new(),
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
}

// Logger interface
impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut msg = match self.log_formatter.msg(record) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Invalid log format: {}", e); 
                self.log_formatter.default_msg(record)
            },
        };
        msg += "\n";

        match &self.file {
            Some(f) => {
                let mut file = f.lock().unwrap();
                file.write_all(msg.as_bytes()).unwrap();
            },
            None => (),
        }
    }

    fn flush(&self) { 
        match &self.file {
            Some(f) => {
                let mut file = f.lock().unwrap();
                file.flush().unwrap();
            },
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

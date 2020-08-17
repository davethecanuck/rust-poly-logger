use log::{LevelFilter};
use chrono::{Datelike, Timelike, Utc};

pub struct TerminalLogger {
    level_filter: LevelFilter,
    
    // EYE change to chrono::format
    timestamp_format: Option<String>,  

    // EYE change to a new data type 
    // e.g. [{timestamp}] {level} [{path}] - {msg}
    // or   [%t] %l [%p] - %m
    msg_format: Option<String>,        
}

impl TerminalLogger {
    pub fn new(level_filter: LevelFilter) -> Self {
        TerminalLogger {
            level_filter: level_filter,
            timestamp_format: None,
            msg_format: None,
        }
    }

    pub fn timestamp_format(&mut self, format: &str) -> &Self {
        self.timestamp_format = Some(String::from(format));
        self
    }

    pub fn msg_format(&mut self, format: &str) -> &Self {
        self.msg_format = Some(String::from(format));
        self
    }
}

// Logger interface
impl log::Log for TerminalLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.level_filter <= metadata.level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
         
        // EYE Optional based on format
        let file = match record.file() {
            Some(f) => f,
            None => "<no_file>",
        };

        // EYE Optional based on format
        let line = match record.line() {
            Some(l) => l,
            None => 0,
        };

        // EYE - use our custom format
        // EYE Optional based on format
        let now = Utc::now();
        println!("[{}-{:02}-{:02} {:02}:{:02}:{:02}.{}] {} [{}:{}] {}", 
                 now.year(), 
                 now.month(), 
                 now.day(), 
                 now.hour(), 
                 now.minute(), 
                 now.second(), 
                 now.second(), // EYE should be microseconds
                 record.metadata().level(), 
                 file,
                 line, 
                 record.args());
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

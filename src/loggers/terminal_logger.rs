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

    // Option to print to stdout instead of stderr (default)
    use_stdout: bool,
}

impl TerminalLogger {
    pub fn new(level_filter: LevelFilter) -> Self {
        TerminalLogger {
            level_filter: level_filter,
            timestamp_format: None,
            msg_format: None,
            use_stdout: false,
        }
    }

    pub fn timestamp_format(&mut self, format: &str) -> &mut Self {
        self.timestamp_format = Some(String::from(format));
        self
    }

    pub fn msg_format(&mut self, format: &str) -> &mut Self {
        self.msg_format = Some(String::from(format));
        self
    }

    pub fn use_stdout(&mut self) -> &mut Self {
        self.use_stdout = true;
        self
    }
}

// Logger interface
impl log::Log for TerminalLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
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
        let msg = format!("[{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}] {} [{}:{}] {}", 
                 now.year(), 
                 now.month(), 
                 now.day(), 
                 now.hour(), 
                 now.minute(), 
                 now.second(), 
                 0, // EYE should be microseconds
                 record.metadata().level(), 
                 file,
                 line, 
                 record.args());

        match self.use_stdout {
            false => eprintln!("{}", msg),
            true => println!("{}", msg),
        }
    }

    fn flush(&self) { }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

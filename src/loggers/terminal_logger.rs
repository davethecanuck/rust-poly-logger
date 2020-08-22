use log::{LevelFilter};

pub struct TerminalLogger {
    level_filter: LevelFilter,
    
    // strftime format string
    timestamp_format: Option<String>,  

    // e.g. [{timestamp}] {level} [{path}] - {msg}
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

    // Set format options
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

    // Retrieve formatted values
    pub fn timestamp(&self) -> String {
        let now = chrono::Local::now();
        match &self.timestamp_format {
            None => {
                // Default format
                now.to_rfc3339()
            },
            Some(f) => {
                now.format(&f).to_string()
            }
        }
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

        let timestamp = self.timestamp();
        let msg = format!("[{}] {} [{}:{}] {}", 
                 timestamp,
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

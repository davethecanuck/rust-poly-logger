use log::{LevelFilter};
use strfmt::strfmt;
use std::collections::HashMap;

pub struct TerminalLogger {
    level_filter: LevelFilter,
    
    // strftime format string
    timestamp_format: &'static str,

    // e.g. [{timestamp}] {level} [{path}] - {msg}
    msg_format: &'static str,

    // Option to print to stdout instead of stderr (default)
    use_stdout: bool,

    // Flag to indicate we need to do more expensive
    // formatting with strfmt
    use_strfmt: bool,
}

impl TerminalLogger {
    pub fn new(level_filter: LevelFilter) -> Self {
        TerminalLogger {
            level_filter: level_filter,
            timestamp_format: "%+",
            msg_format: "",
            use_stdout: false,
            use_strfmt: false,
        }
    }

    // Set format options
    pub fn timestamp_format(&mut self, format: &'static str) -> &mut Self {
        self.timestamp_format = format;
        self
    }

    pub fn msg_format(&mut self, format: &'static str) -> &mut Self {
        // Using custom format
        self.use_strfmt = true;
        self.msg_format = format;
        self
    }

    pub fn use_stdout(&mut self) -> &mut Self {
        self.use_stdout = true;
        self
    }

    // Format value accessors
    fn timestamp(&self) -> String {
        match &self.timestamp_format {
            &"" => {
                "".to_string()
            },
            f => {
                let now = chrono::Local::now();
                now.format(&f).to_string()
            }
        }
    }

    fn line(&self, record: &log::Record) -> u32 {
        match record.line() {
            Some(l) => l,
            None => 0,
        }
    }

    fn file(&self, record: &log::Record) -> String {
        match record.file() {
            Some(f) => f.to_string(),
            None => "<no_file>".to_string(),
        }
    }

    fn default_msg(&self, record: &log::Record) -> String {
        format!(
            "[{timestamp}] {level} [{file}:{line}] {args}", 
            timestamp=self.timestamp(),
            level=record.metadata().level(), 
            file=self.file(record),
            line=self.line(record),
            args=record.args())
    }

    fn msg(&self, record: &log::Record) -> String {
        let mut vars = HashMap::new();
        vars.insert("timestamp".to_string(), self.timestamp());
        vars.insert("level".to_string(), 
                    record.metadata().level().to_string());
        vars.insert("file".to_string(), self.file(record));
        vars.insert("line".to_string(), self.line(record).to_string());
        vars.insert("args".to_string(), record.args().to_string());

        /* EYE - TBD - Return Result<String> 
         * from function so I can pass error to implementor
         * if using MsgFormatter
         */
        match strfmt(self.msg_format, &vars) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Invalid logger format '{}': {}", 
                         self.msg_format, e);
                self.default_msg(record)
            },
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

        // EYE - replace with MsgFormatter object
        // so we can use in other loggers

        // NOTE - Use strfmt only if custom message
        // as it's more expensive
        let msg = match self.use_strfmt {
            false => {
                self.default_msg(record)
            },
            true => {
                self.msg(record)
            },
        };

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

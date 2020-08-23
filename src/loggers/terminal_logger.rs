use log::{LevelFilter, SetLoggerError};
use super::LogFormatter;

pub struct TerminalLogger {
    level_filter: LevelFilter,
    
    // MsgFormatter does most of the work
    // for our logger implementations
    pub log_formatter: LogFormatter,
    
    // Option to print to stdout instead of stderr (default)
    use_stdout: bool,
}

impl TerminalLogger {
    pub fn new(level_filter: LevelFilter) -> Self {
        TerminalLogger {
            level_filter: level_filter,
            log_formatter: LogFormatter::new(),
            use_stdout: false,
        }
    }

    // Initialize log as boxed logger
    pub fn init(logger: TerminalLogger) -> Result<(), SetLoggerError> {
        log::set_max_level(logger.level_filter);
        log::set_boxed_logger(Box::new(logger))
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

    pub fn use_stdout(&mut self) -> &mut Self {
        self.use_stdout = true;
        self
    }

    pub fn use_stderr(&mut self) -> &mut Self {
        self.use_stdout = false;
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

        let msg = match self.log_formatter.msg(record) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Invalid log format: {}", e); 
                self.log_formatter.default_msg(record)
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

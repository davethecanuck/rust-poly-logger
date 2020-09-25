use std::path::Path;
use std::fs::{File, OpenOptions};
use log::{LevelFilter, SetLoggerError};
use super::gen_logger::{GenLogger};
use super::log_formatter::LogFormatter;

// NOTE - These are really builder classes for returning
// a GenLogger. We use new() and chained methods
// to set the properties. E.g. We may want to set some 
// properties in the FileLogger like 'truncate' or 'file name'
// or 'auto file name version' before passing on to 
// the GenLogger. 

//-------------------------------------------------
pub struct StdoutLogger {}

impl StdoutLogger {
    pub fn new(level_filter: LevelFilter) -> GenLogger<std::io::Stdout> {
        GenLogger::new(level_filter, std::io::stdout())
    }
}

//-------------------------------------------------
pub struct StderrLogger {}

impl StderrLogger {
    pub fn new(level_filter: LevelFilter) -> GenLogger<std::io::Stderr> {
        GenLogger::new(level_filter, std::io::stderr())
    }
}

//-------------------------------------------------
pub struct FileLogger {
    level_filter: LevelFilter,
    truncate: bool,
    filename: Option<String>,
    log_formatter: LogFormatter,
}

impl FileLogger {
    // In this case we return a FileLogger, but use 
    // create() to get the GenLogger<File> 
    pub fn new(level_filter: LevelFilter) -> FileLogger {
        FileLogger { 
            level_filter, 
            truncate: false,
            filename: None,
            log_formatter: LogFormatter::new()
        }
    }

    // Intercept init() and call create() to get the 
    // GenLogger instance to pass into log:: calls
    pub fn init(&self) -> Result<(), SetLoggerError> {
        let logger = self.create();
        log::set_max_level(self.level_filter);
        log::set_boxed_logger(Box::new(logger))
    }

    // Expose LogFormatter options
    pub fn timestamp_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.timestamp_format(format);
        self
    }

    pub fn msg_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.msg_format(format);
        self
    }

    // Expose OpenOptions
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    pub fn filename(&mut self, filename: &'static str) -> &mut Self {
        self.filename = Some(filename.to_string());
        self
    }

    // We need to call this to get a Log interface 
    // object such as when  passing to PolyLogger.
    // If this is a standalone logger, create() will be
    // called when do the init()
    pub fn create(&self) -> GenLogger<File> {
        match &self.filename {
            Some(filename) => {
                // Create the log file parent directory
                let path = Path::new(&filename);
                let parent = path.parent().unwrap();
                std::fs::create_dir_all(parent).unwrap();
                let mut file = OpenOptions::new();
                let file = match &self.truncate {
                    true => file.write(true).truncate(true),
                    false => file.append(true),
                };
                let file = file.create(true)
                    .open(path)
                    .unwrap();

                // Create the GenLogger and pass in the log_formatter
                let mut logger = GenLogger::new(self.level_filter, file);
                logger.set_log_formatter(&self.log_formatter);
                logger
            },
            _ => {
                // NOTE - Can we do better than panic?
                panic!("No filename specified for FileLogger");
            }
        }
    }
}
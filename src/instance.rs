use std::path::Path;
use std::fs::{File, OpenOptions};
use log::{LevelFilter, SetLoggerError};
use super::gen_logger::{GenLogger};
use super::log_formatter::LogFormatter;

/// Implements a GenLogger that outputs to Stdout
///
/// # Simple Example
/// Instantiates a logger with default message and timestamp format
/// ```
/// use log::info;
/// use log::LevelFilter;
/// use poly_logger::StdoutLogger;
///
/// let mut logger = StdoutLogger::new(LevelFilter::Info);
/// logger.init().unwrap();
/// info!("This is an INFO message to STOUT");
/// ```
///
/// # Customized Example
/// Instantiates a logger with custom message and timestamp format
/// ```
/// use log::info;
/// use log::LevelFilter;
/// use poly_logger::StdoutLogger;
/// 
/// let mut logger = StdoutLogger::new(LevelFilter::Info);
/// logger.timestamp_format("%X%.6f")
///       .msg_format("[{timestamp} {file}:{line}] - {level} - {args}");
/// logger.init().unwrap();
/// info!("This is a formatted INFO message to STOUT");
/// ```
///
/// # Note
/// The call to new() is actually returning an instance of GenLogger<Stdout>.
/// The StdoutLogger struct is just a way to instantiate a GenLogger with 
/// Stdout as the target for output.
/// See [GenLogger](struct.GenLogger.html) for the full list of methods that
/// can be called on the logger instance.
///
pub struct StdoutLogger {}

impl StdoutLogger {
    pub fn new(level_filter: LevelFilter) -> GenLogger<std::io::Stdout> {
        GenLogger::new(level_filter, std::io::stdout())
    }
}

/// Implements a GenLogger that outputs to Stderr
///
/// # Simple Example
/// Instantiates a logger with default message and timestamp format
/// ```
/// use log::info;
/// use log::LevelFilter;
/// use poly_logger::StderrLogger;
///
/// let mut logger = StderrLogger::new(LevelFilter::Info);
/// logger.init().unwrap();
/// info!("This is an INFO message");
/// ```
///
/// # Customized Example 
/// Implements with custom timestamp format and message format
/// ```
/// use log::info;
/// use log::LevelFilter;
/// use poly_logger::StderrLogger;
///
/// let mut logger = StderrLogger::new(LevelFilter::Info);
/// logger.timestamp_format("%X%.6f")
///       .msg_format("[{timestamp} {file}:{line}] - {level} - {args}");
/// logger.init().unwrap();
/// info!("This is an INFO message with custom formatting");
/// ```
///
/// # Note
/// The call to new() is actually returning an instance of GenLogger<Stderr>.
/// The StderrLogger struct is just a way to instantiate a GenLogger 
/// with Stderr as the target for output.
/// See [GenLogger](struct.GenLogger.html) for the full list of methods that
/// can be called on the logger instance.
///
pub struct StderrLogger {}

impl StderrLogger {
    pub fn new(level_filter: LevelFilter) -> GenLogger<std::io::Stderr> {
        GenLogger::new(level_filter, std::io::stderr())
    }
}

/// Implements a GenLogger that outputs to File
///
/// # Simple Example
/// Instantiates a logger with default message and timestamp format
/// and log file written in 'append' mode (as opposed to truncating first).
/// ```
/// use log::info;
/// use log::LevelFilter;
/// use poly_logger::FileLogger;
///
/// // Currently it is mandatory to set the filename, but I may 
/// // introduce a default where an auto-named file is created
/// // in the current directory
/// let mut logger = FileLogger::new(LevelFilter::Info);
/// logger.filename("./test.log");  
/// logger.init().unwrap();
/// info!("This is an INFO message");
/// ```
///
/// # Customized Example
/// Instantiates a logger with custom timestamp and message format. 
/// We are also setting the truncate flag so that log file is 
/// overwritten rather than appended to.
/// ```
/// use log::info;
/// use log::LevelFilter;
/// use poly_logger::FileLogger;
///
/// let mut logger = FileLogger::new(LevelFilter::Info);
/// logger.timestamp_format("%F %X%.3f %Z")
///       .msg_format("{level} [{timestamp} {file}:{line}] - {args}")
///       .truncate(false)
///       .filename("./test.log");
/// logger.init().unwrap();
/// info!("This is an INFO message with custom formatting");
/// ```
///
/// # Note
/// The call to new() is actually returning an instance of GenLogger<Stderr>.
/// The StderrLogger struct is just a way to instantiate a GenLogger 
/// with Stderr as the target for output.
/// See [GenLogger](struct.GenLogger.html) for the full list of methods that
/// can be called on the logger instance.
///
pub struct FileLogger {
    level_filter: LevelFilter,
    truncate: bool,
    filename: Option<String>,
    log_formatter: LogFormatter,
}

impl FileLogger {
    /// Unlike the StderrLogger or StdoutLogger we return a 
    /// FileLogger struct instead of a GenLogger struct as
    /// we need to specify file-specific options after creation.
    /// A call to create() returns the GenLogger<File> instance
    /// we need, though this is typically done by calling init().
    pub fn new(level_filter: LevelFilter) -> FileLogger {
        FileLogger { 
            level_filter, 
            truncate: false,
            filename: None,
            log_formatter: LogFormatter::new()
        }
    }

    /// Calls create() to get the GenLogger instance 
    /// which is then in turn initialized. 
    pub fn init(&self) -> Result<(), SetLoggerError> {
        let logger = self.create();
        log::set_max_level(self.level_filter);
        log::set_boxed_logger(Box::new(logger))
    }

    /// Sets timestamp format for the underlying 
    /// [GenLogger](struct.GenLogger.html) instance
    pub fn timestamp_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.timestamp_format(format);
        self
    }

    /// Sets message format for the underlying 
    /// [GenLogger](struct.GenLogger.html) instance
    pub fn msg_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.msg_format(format);
        self
    }

    /// Truncates log file before writing. Default is to append
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    /// Sets log file name
    pub fn filename(&mut self, filename: &'static str) -> &mut Self {
        self.filename = Some(filename.to_string());
        self
    }

    /// We need to call this to get a Log interface 
    /// object such as when  passing to PolyLogger.
    /// If this is a standalone logger, create() will be
    /// called when do the init().
    pub fn create(&self) -> GenLogger<File> {
        match &self.filename {
            Some(filename) => {
                // Create the file parent directory
                let path = Path::new(&filename);
                let parent = path.parent().unwrap();
                std::fs::create_dir_all(parent).unwrap();

                // Create the log file with our truncate option
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
                //logger.set_log_formatter(&self.log_formatter);
                logger.log_formatter = self.log_formatter.clone();
                logger
            },
            _ => {
                // NOTE - Can we do better than panic?
                panic!("No filename specified for FileLogger");
            }
        }
    }
}

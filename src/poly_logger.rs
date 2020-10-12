use log::{Level, SetLoggerError, MetadataBuilder};
use std::fmt;

/// Implements a super-logger that can redirect to other loggers
///
/// This is useful if you want to log some messages to STDERR or 
/// STDOUT and some to File in perhaps a different format. When 
/// using PolyLogger, you create any number of child loggers and
/// add them to the PolyLogger. Then you set the PolyLogger instance
/// as the only one to be passed to the log system via init().
///
/// # Example
/// Create a file logger with the default timestamp/message format, 
/// and also a raw logger for debug messages which gets sent to
/// STDERR.
/// ```
/// use log::{debug,info,LevelFilter};
/// use poly_logger::{FileLogger,StderrLogger,PolyLogger};
///
/// let mut stderr_log = StderrLogger::new(LevelFilter::Debug);
/// stderr_log.msg_format("{args}");
///
/// let mut file_log = FileLogger::new(LevelFilter::Info);
/// file_log.filename("./test.log");
///
/// let mut poly_log = PolyLogger::new();
/// poly_log.add(file_log.create()); // create() returns the GenLogger
/// poly_log.add(stderr_log);
/// poly_log.init();
///
/// info!("This goes to both loggers");
/// debug!("This only goes to stderr");
/// ```
///
pub struct PolyLogger {
    loggers: Vec<Box<dyn log::Log>>,
    max_level: Level,
}

impl PolyLogger {
    /// Instantiate a new PolyLogger
    pub fn new() -> Self {
        PolyLogger{loggers: Vec::new(), max_level: Level::Error}
    }
    
    /// Initializes the log interface using this PolyLogger
    /// as a boxed logger. This moves self so is the last
    /// method to call on this object.
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.max_level.to_level_filter());
        log::set_boxed_logger(Box::new(self))
    }

    /// Returns the maximum log Level of any of the child
    /// loggers added to this PolyLogger
    pub fn max_level(&self) -> Level {
        self.max_level
    }

    /// Adds a logger to this PolyLogger. This can be anything that
    /// implements the [log::Log](https://docs.rs/log/0.4.11/log/) interface.
    pub fn add<T: log::Log + 'static>(&mut self, logger: T) {
        // Capture the max level before boxing into our
        // vector of loggers.
        let test_levels = vec![Level::Warn, Level::Info, Level::Debug, Level::Trace];
        let mut builder = MetadataBuilder::new();

        for level in test_levels {
            if level > self.max_level {
                builder.level(level);
                let metadata = builder.build();

                if logger.enabled(&metadata) {
                    self.max_level = level;
                }
            }
        }

        // Finally add to our vector
        self.loggers.push(Box::new(logger));
    }
}

impl fmt::Debug for PolyLogger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PolyLogger[ loggers:{} ]", self.loggers.len())
    }
}

impl log::Log for PolyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &log::Record) {
        // Each logger checks if enabled in the log() call
        self.loggers.iter().for_each(|logger| { 
            logger.log(record); 
        });
    }

    fn flush(&self) {
        self.loggers.iter().for_each(|logger| { 
            logger.flush();
        });
    }
}

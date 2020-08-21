// Import our loggers module
pub mod loggers;

use log::{Level, SetLoggerError, MetadataBuilder};
use std::fmt;

// Initialize log as boxed logger
pub fn init(pl: PolyLogger) -> Result<(), SetLoggerError> {
    log::set_max_level(pl.max_level.to_level_filter());
    log::set_boxed_logger(Box::new(pl))
}

// PolyLogger contains a list of child Log instances
pub struct PolyLogger {
    loggers: Vec<Box<dyn log::Log>>,
    max_level: Level,
}

impl PolyLogger {
    pub fn new() -> Self {
        PolyLogger{loggers: Vec::new(), max_level: Level::Error}
    }

    pub fn max_level(&self) -> Level {
        self.max_level
    }

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

// Logger interface
impl log::Log for PolyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &log::Record) {
        // Each logger should check if enabled in the log() call
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

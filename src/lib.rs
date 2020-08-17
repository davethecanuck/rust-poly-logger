// Declare our submodule
pub mod loggers;

use std::fmt;
use log::SetLoggerError;

// Initialize log as boxed logger
pub fn init(pl: PolyLogger) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(pl))
}

// PolyLogger contains a list of child Log instances
pub struct PolyLogger {
    loggers: Vec<Box<dyn log::Log>>,
}

impl PolyLogger {
    pub fn new() -> Self {
        PolyLogger{loggers: Vec::new()}
    }

    /* Not sure if I can do this...
    pub fn init(&self) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(Self{self.loggers}))
    }
    */

    pub fn add<T: log::Log + 'static>(&mut self, logger: T) {
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
        // EYE - Or skip this check and just check in each
        // logger in log() call
        self.loggers.iter().any(|logger| { 
            logger.enabled(metadata)
        })
    }

    fn log(&self, record: &log::Record) {
        // EYE - Do I need to check enabled for each? Or does Log do this?
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

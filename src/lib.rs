use log::{SetLoggerError, LevelFilter};
use chrono::{Datelike, Timelike, Utc};

// Empty struct
pub struct PubSubLogger;

// Single static instance
static LOGGER: PubSubLogger = PubSubLogger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    // EYE - In actuality, we want to create a logger that 
    // routes messages to subscriber sinks (files, stderr, Kafka, 
    // or whatever). We might do that here or in 'addSubscriber' 
    // such that the set_logger call is only done once
    
    // Boxed logger
    // EYE - Do simple/flexi/etc. loggers use Box or static?
    //log::set_boxed_logger(Box::new(PubSubLogger))
    
    // Static logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level))
}

// Logger interface
impl log::Log for PubSubLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
         
        let file = match record.file() {
            Some(f) => f,
            None => "<no_file>",
        };

        let line = match record.line() {
            Some(l) => l,
            None => 0,
        };

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
        
        /*
          This returns:

          INFO:main => Record { 
              metadata: Metadata { 
                  level: Info, 
                  target: "main" 
              }, 
              args: This is an INFO message, 
              module_path: Some(Static("main")), 
              file: Some(Static("examples/main.rs")), 
              line: Some(8) } 
              -- This is an INFO message 

        println!("{}:{} => {:?} -- {} ",
                record.level(),
                record.target(),
                record,
                record.args());
       */
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

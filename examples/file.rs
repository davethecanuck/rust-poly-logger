use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::{FileLogger};

fn main() {
    let mut logger = FileLogger::new(LevelFilter::Info);
    let filename = "/tmp/test.log";
    println!("Logging to {}", filename);
    logger.timestamp_format("%F %X%.3f %Z")
          .msg_format("{level} [{timestamp} {file}:{line}] - {args}")
          .truncate(false)
          .filename(filename);
    logger.init().unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


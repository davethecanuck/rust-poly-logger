use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::StdoutLogger;

fn main() {
    let mut logger = StdoutLogger::new(LevelFilter::Info);

    logger.timestamp_format("%F %X%.3f %Z")
          .msg_format("{level} [{timestamp} {file}:{line}] - {args}");
    logger.init().unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


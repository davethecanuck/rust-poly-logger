use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::loggers::TerminalLogger;

fn main() {
    // Create some logger instances
    let mut tl1 = TerminalLogger::new(LevelFilter::Info);
    tl1.timestamp_format("%F %X%.3f %Z");
    tl1.msg_format("{level} [{timestamp} {file}:{line}] - {args}");
    tl1.init().unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


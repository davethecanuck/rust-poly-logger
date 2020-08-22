use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::loggers::TerminalLogger;

fn main() {
    // Create some logger instances
    let mut tl1 = TerminalLogger::new(LevelFilter::Info);
    tl1.timestamp_format("%F %X%.3f %Z");
    tl1.msg_format("[{timestamp}] {level} [{file}:{line}] - {args}");
    log::set_boxed_logger(Box::new(tl1)).unwrap();
    log::set_max_level(LevelFilter::Trace);

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


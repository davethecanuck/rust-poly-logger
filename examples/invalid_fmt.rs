use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::loggers::TerminalLogger;

fn main() {
    // Create some logger instances
    let mut tl1 = TerminalLogger::new(LevelFilter::Info);
    tl1.msg_format("{foo} [{timestamp} {file}:{line}] - {baz}");
    TerminalLogger::init(tl1).unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


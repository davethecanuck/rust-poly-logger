use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::StderrLogger;

fn main() {
    let mut logger = StderrLogger::new(LevelFilter::Info);
    logger.msg_format("{foo} [{timestamp} {file}:{line}] - {baz}");
    logger.init().unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


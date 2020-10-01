use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::GenLogger;

fn main() {
    let logger = GenLogger::new(LevelFilter::Info, std::io::stderr());
    logger.init().unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


use log::{trace,debug,info,warn,error};
use log::LevelFilter;

fn main() {
    pubsub_logger::init(LevelFilter::Info).unwrap();
    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


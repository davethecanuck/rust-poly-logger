use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::loggers::terminal_logger::TerminalLogger;
use poly_logger::PolyLogger;

fn main() {
    // Create some logger instances
    let tl1 = TerminalLogger::new(LevelFilter::Info)
        .timestamp_format("%a %b %e %T %Y")
        .msg_format("[{timestamp}] {level} [{file}:{line}] - {args}");

    let tl2 = TerminalLogger::new(LevelFilter::Info)
        .timestamp_format("%a %b %e %T")
        .msg_format("[{timestamp}] {level} - {args}");
    
    // Create the poly logger and add our 
    // logger instances
    let pl = PolyLogger::new();
    pl.add(tl1);
    pl.add(tl2);

    // Either of these work
    // poly_logger::init(pl).unwrap();
    log::set_boxed_logger(Box::new(pl));

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


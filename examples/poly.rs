use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::loggers::TerminalLogger;
use poly_logger::PolyLogger;


fn main() {
    // Create some logger instances
    let mut tl1 = TerminalLogger::new(LevelFilter::Warn);
    tl1.timestamp_format("%a %b %e %T %Y")
       .msg_format("[{timestamp}] {level} [{file}:{line}] - {args}")
       .use_stdout();

    // Default formatting
    let mut tl2 = TerminalLogger::new(LevelFilter::Info);
    tl2.msg_format("{foo} {baz}");

    let mut tl3 = TerminalLogger::new(LevelFilter::Debug);
    tl3.msg_format("MSG: {level} - {args}");
    tl3.timestamp_format("");

    // EYE - TBD logger.json_format()
    
    // Create the poly logger and add our logger instances
    let mut pl = PolyLogger::new();
    pl.add(tl1);
    pl.add(tl2);
    pl.add(tl3);

    // Logger init is the last step
    poly_logger::init(pl).unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}


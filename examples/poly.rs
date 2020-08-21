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

    let mut tl2 = TerminalLogger::new(LevelFilter::Info);
    tl2.timestamp_format("%a %b %e %T")
       .msg_format("[{timestamp}] {level} - {args}");

    let tl3 = TerminalLogger::new(LevelFilter::Debug);
    
    // Create the poly logger and add our logger instances
    let mut pl = PolyLogger::new();
    println!("0. max_level={}", pl.max_level());
    pl.add(tl1);
    println!("1. max_level={}", pl.max_level());
    pl.add(tl2);
    println!("2. max_level={}", pl.max_level());
    pl.add(tl3);
    println!("3. max_level={}", pl.max_level());

    // Logger init is the last step
    poly_logger::init(pl).unwrap();

    trace!("This is an TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");
}

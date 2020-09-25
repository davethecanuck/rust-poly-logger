use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::{PolyLogger,StderrLogger,StdoutLogger,FileLogger};

fn main() {
    // Create some logger instances and run all through
    // the PolyLogger
    
    // Default format 
    let tl0 = StderrLogger::new(LevelFilter::Info);

    // Custom format
    let mut tl1 = StderrLogger::new(LevelFilter::Warn);
    tl1.timestamp_format("%a %b %e %T %Y")
       .msg_format("Custom: [{timestamp}] {level} [{file}:{line}] - {args}");

    // Simpler format
    let mut tl2 = StderrLogger::new(LevelFilter::Info);
    tl2.msg_format("Simple1: {level} [{timestamp}] {args}")
       .timestamp_format("%T");

    // Even simpler
    let mut tl3 = StderrLogger::new(LevelFilter::Debug);
    tl3.msg_format("Simple2: {level} - {args}")
       .timestamp_format("");

    // Raw format to stdout
    let mut tl4 = StdoutLogger::new(LevelFilter::Trace);
    tl4.msg_format("{args}")
       .timestamp_format("");

    // File logger
    let filename = "/tmp/file_logger.log";
    let mut fl0 = FileLogger::new(LevelFilter::Info);
    fl0.msg_format("Simple1: {level} [{timestamp}] {args}")
       .timestamp_format("%T")
       .filename(filename);
    println!("Logging to {}", filename);
    
    // Create the poly logger and add our logger instances
    let mut pl = PolyLogger::new();
    pl.add(tl0);
    pl.add(tl1);
    pl.add(tl2);
    pl.add(tl3);
    pl.add(tl4);
    pl.add(fl0.create());
    pl.init().unwrap();

    trace!("This is an TRACE message");
    eprintln!("------------------------------");
    debug!("This is a DEBUG message");
    eprintln!("------------------------------");
    info!("This is an INFO message");
    eprintln!("------------------------------");
    warn!("This is a WARN message");
    eprintln!("------------------------------");
    error!("This is an ERROR message");
}


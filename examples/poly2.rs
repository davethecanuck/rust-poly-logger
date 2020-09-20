use log::{trace,debug,info,warn,error};
use log::LevelFilter;
use poly_logger::{PolyLogger,StderrLogger};

fn main() {
    // Create a few terminal loggers
    let mut tl0 = StderrLogger::new(LevelFilter::Debug);
    tl0.msg_format("TL0: {timestamp} - {level} - {args}");

    let mut tl1 = StderrLogger::new(LevelFilter::Info);
    tl1.msg_format("TL1: {timestamp} - {level} - {args}");

    let mut tl2 = StderrLogger::new(LevelFilter::Warn);
    tl2.msg_format("TL2: {timestamp} - {level} - {args}");

    // Put one one terminal logger into one poly_logger
    let mut pl0 = PolyLogger::new();
    pl0.add(tl0);

    // Put other terminal loggers into another poly_logger
    let mut pl1 = PolyLogger::new();
    pl1.add(tl1);
    pl1.add(tl2);

    // Put second poly_logger into the first poly_logger
    // which is the one we actually register with the 
    // log system via init().
    // Not sure if this would ever make sense, but hey...
    pl0.add(pl1);
    pl0.init().unwrap();

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


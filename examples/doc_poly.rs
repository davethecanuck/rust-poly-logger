use log::{debug,info,LevelFilter};
use poly_logger::{FileLogger,StderrLogger,PolyLogger};

fn main() {
    let mut stderr_log = StderrLogger::new(LevelFilter::Debug);
    stderr_log.msg_format("{args}");

    let mut file_log = FileLogger::new(LevelFilter::Info);
    file_log.filename("./test.log");

    let mut poly_log = PolyLogger::new();
    poly_log.add(file_log.create()); // create() returns the GenLogger
    poly_log.add(stderr_log);
    poly_log.init().unwrap();

    info!("This goes to both loggers");
    debug!("This only goes to stderr");
}

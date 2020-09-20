// Import our loggers module
pub mod log_formatter;
pub use log_formatter::LogFormatter;
pub mod poly_logger;
pub use poly_logger::PolyLogger;
pub mod gen_logger;
pub use gen_logger::GenLogger;
pub mod instance;
pub use instance::{StdoutLogger,StderrLogger,FileLogger};

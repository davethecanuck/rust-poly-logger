//! Top level crate doc
//! * here
//! * and here

// Private module used in other loggers
mod log_formatter;

// Import our loggers module
mod poly_logger;
pub use poly_logger::PolyLogger;
mod gen_logger;
pub use gen_logger::GenLogger;
mod instance;
pub use instance::{StdoutLogger,StderrLogger,FileLogger};

// Bring our logger implementations into the 'loggers' namespace
// so clients can reference: 
// poly_logger::loggers::TerminalLogger;
mod terminal_logger;
pub use terminal_logger::TerminalLogger;

// Only used privately within loggers
mod log_formatter;
pub use log_formatter::LogFormatter;

//! # Overview 
//! This crate provides several useful 
//! [log](https://docs.rs/log/0.4.11/log/) interface implementations. 
//!
//! [GenLogger](struct.GenLogger.html) is a generic logger 
//! which allows custom message and time formats for a logger. 
//! The target for the log messages can by anything implementing 
//! Write + Sync + Send + 'static, such as 
//! [std::fs::File](https://doc.rust-lang.org/std/fs/struct.File.html)
//! [std::io::Stderr](https://doc.rust-lang.org/std/io/struct.Stderr.html)
//! or
//! [std::io::Stdout](https://doc.rust-lang.org/std/io/struct.Stdout.html)
//!
//! [StdoutLogger](struct.StdoutLogger.html), 
//! [StderrLogger](struct.StderrLogger.html), 
//! and [FileLogger](struct.FileLogger.html) are wrappers for 
//! [GenLogger](struct.GenLogger.html) which each implement
//! output to their respective sink types.
//!
//! [PolyLogger](struct.PolyLogger.html)
//! is a container for other loggers. For example, you may
//! want to log certain messages to STDERR in one format, and others
//! to go to a File in a different format. You simply create a FileLogger, 
//! a StderrLogger, and then add both to a new PolyLogger.
//!
//! In each logger class, you call init() to assign the class instance
//! to be the Log implementor for your application. 
//!
//! The examples directory provides use cases for each of the classes.
//!
//! # Notes
//! The author is new to Rust and this is an early release of this crate. 
//! This crate will evolve as my familiarity with the language
//! grows, and as I start using this crate in my own projects. I will
//! however not break the current API without a major revision number
//! change.
//!
//! # Future Work
//! * Reduce the use of unwrap() and do some proper error handling/propagation
//! * Modify PolyLogger to use producer/consumer queue to minimize
//!   the cost of logging in the main application thread
//! * Possibly add producer/consumer queue to GenLogger
//! * Add unit tests 
//! * Add options to FileLogger to create an auto-naming and/or
//!   auto-incrementing file naming convention. E.g. Perhaps set 
//!   the filename to be "log.{yyyymmdd}.{hhmmss}". I.e. Add the 
//!   same flexibility we have for the log messages to the log
//!   file name itself.
//! * Look into replacement for [strfmt](https://docs.rs/strfmt/0.1.6/strfmt/)
//!   for message formatting. Perhaps include some statically defined canned 
//!   options or wait for a more efficient implementation when the rust 
//!   language allows it.
//!

// Private module used in other loggers
mod log_formatter;

// Import our loggers module
mod poly_logger;
pub use crate::poly_logger::PolyLogger;
mod gen_logger;
pub use gen_logger::GenLogger;
mod instance;
pub use instance::{StdoutLogger,StderrLogger,FileLogger};

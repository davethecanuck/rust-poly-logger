use std::io::{Write};
use std::sync::{Mutex};
use log::{LevelFilter, SetLoggerError};
use super::log_formatter::LogFormatter;

/// Implements a generic logger for use with different types of writers
/// 
/// # Simple Example
/// Instantiates a STDERR logger with default message and timestamp format
/// ```
/// use log::{info,LevelFilter};
/// use poly_logger::GenLogger;
///
/// let mut logger = GenLogger::new(LevelFilter::Info, std::io::stderr());
/// logger.init().unwrap();
/// info!("This is an INFO message");
/// ```
///
/// # Customized Example 
/// Instantiates a STDOUT logger with custom timestamp format and message format
/// ```
/// use log::{info,LevelFilter};
/// use poly_logger::GenLogger;
///
/// let mut logger = GenLogger::new(LevelFilter::Info, std::io::stdout());
/// logger.timestamp_format("%X%.6f")
///       .msg_format("[{timestamp} {file}:{line}] - {level} - {args}");
/// logger.init().unwrap();
/// info!("This is a custom INFO message");
/// // Output is something like: 
/// // [20:52:57.909459 examples/stdout.rs:14] - INFO - This is a custom INFO message
/// ```
///
pub struct GenLogger<T: Write + Sync + Send + 'static> {
    level_filter: LevelFilter,
    pub log_formatter: LogFormatter,
    writer: Mutex<T>,
}

impl<T> GenLogger<T>
where T: Write + Sync + Send + 'static {
    /// Instantiate a new GenLogger with the given log level
    /// filter and a Writer instance
    pub fn new(level_filter: LevelFilter, writer: T) -> Self {
        GenLogger {
            level_filter: level_filter,
            log_formatter: LogFormatter::new(),
            writer: Mutex::new(writer),
        }
    }

    /// Initializes the log interface using this GenLogger
    /// as a boxed logger. This moves self so is the last 
    /// method to call on this object.
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level_filter);
        log::set_boxed_logger(Box::new(self))
    }

    /// Sets the timestamp format to use in our log messages.
    ///
    /// The format string may be any valid format from the
    /// [chrono::format::strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)
    /// module.
    ///
    /// If format is set to "" the timestamp will not be 
    /// calculated/formatted, which is more efficient if 
    /// if timestamp is not part of the message format.
    ///
    /// The default timestamp format of '%+' (ISO 8601 / RFC 3339 date & time format)
    /// will be used if you do not call timestamp_format(<format>) on your logger.
    ///
    pub fn timestamp_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.timestamp_format(format);
        self
    }

    /// Sets the format for message written by our logger
    /// 
    /// The format can use any combination of the following placeholders
    /// * {timestamp} - Date/time stamp of this message
    /// * {level} - The [log::Level](https://docs.rs/log/0.4.1/log/enum.Level.html)
    ///           for this message
    /// * {file} - The Rust source file where the log message was generated
    /// * {line} - The line in the Rust source file where the log message was generated
    /// * {args} - The log message itself
    ///
    /// Note that the names of the placeholders come from the corresponding 
    /// definitions in [log::Record](https://docs.rs/log/0.4.4/log/struct.Record.html).
    /// 
    /// This method relies on the 
    /// [strfmt](https://docs.rs/strfmt/0.1.6/strfmt/) crate which gives us 
    /// flexibility at the expense of being somewhat expensive. It's our best
    /// option as of late 2020 though.
    ///
    /// The default format is:
    /// [{timestamp}] {level} [{file}:{line}] {args}
    /// 
    pub fn msg_format(&mut self, format: &'static str) -> &mut Self {
        self.log_formatter.msg_format(format);
        self
    }
}

// Logger interface
impl<T> log::Log for GenLogger<T>
where T: Write + Sync + Send + 'static {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = match self.log_formatter.msg(record) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Invalid log format: {}", e); 
                self.log_formatter.default_msg(record)
            },
        };

        // Note: may want option to not include newline
        let mut w = self.writer.lock().unwrap();
        w.write_all((msg + "\n").as_bytes()).unwrap();
    }

    fn flush(&self) { 
        let mut w = self.writer.lock().unwrap();
        w.flush().unwrap();
    }
}


use log::{LevelFilter};
use std::path::Path;
use std::fs::{OpenOptions, File};
use super::gen_logger::{GenLogger};

// EYE - These are really builder classes for returning
// a GenLogger. Perhaps should use build() or similar to
// return the GenLogger, but use new() and chained methods
// to set the properties. E.g. We may want to set some 
// properties in the FileLogger like 'append' or 'file name'
// or 'auto file name version' before passing on to 
// the GenLogger

//-------------------------------------------------
pub struct StdoutLogger {}

impl StdoutLogger {
    pub fn new(level_filter: LevelFilter) -> GenLogger<std::io::Stdout> {
        GenLogger::new(level_filter, std::io::stdout())
    }
}

//-------------------------------------------------
pub struct StderrLogger {}

impl StderrLogger {
    pub fn new(level_filter: LevelFilter) -> GenLogger<std::io::Stderr> {
        GenLogger::new(level_filter, std::io::stderr())
    }
}

//-------------------------------------------------
pub struct FileLogger {}

impl FileLogger {
    pub fn new(level_filter: LevelFilter, filename: &'static str) -> GenLogger<File> {
        // Create the log file parent directory
        let path = Path::new(filename);
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();

        // EYE - externalize to options
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        GenLogger::new(level_filter, file)
    }
}


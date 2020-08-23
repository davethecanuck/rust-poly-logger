use strfmt::strfmt;
use std::collections::HashMap;

pub struct LogFormatter {
    // strftime format string
    timestamp_format: &'static str,

    // e.g. [{timestamp}] {level} [{path}] - {msg}
    msg_format: &'static str,

    // Flag to indicate we need to do more expensive
    // formatting with strfmt
    use_strfmt: bool,
}

// NOTE: Using default error type
type MsgResult = Result<String, std::fmt::Error>;

impl LogFormatter {
    pub fn new() -> Self {
        LogFormatter {
            timestamp_format: "%+",
            msg_format: "",
            use_strfmt: false,
        }
    }

    // Set format options
    pub fn timestamp_format(&mut self, format: &'static str) -> &mut Self {
        self.timestamp_format = format;
        self
    }

    pub fn msg_format(&mut self, format: &'static str) -> &mut Self {
        // Using custom format
        self.use_strfmt = true;
        self.msg_format = format;
        self
    }

    // Format value accessors
    pub fn msg(&self, record: &log::Record) -> MsgResult {
        // NOTE - Use strfmt only if custom message
        // as it's more expensive. We could add various
        // canned defaults for performance. 
        match self.use_strfmt {
            false => {
                Ok(self.default_msg(record))
            },
            true => {
                self.custom_msg(record)
            },
        }
    }

    pub fn default_msg(&self, record: &log::Record) -> String {
        format!(
            "[{timestamp}] {level} [{file}:{line}] {args}", 
            timestamp=self.timestamp(),
            level=record.metadata().level(), 
            file=self.file(record),
            line=self.line(record),
            args=record.args())
    }

    fn custom_msg(&self, record: &log::Record) -> MsgResult {
        let mut vars = HashMap::new();
        vars.insert("timestamp".to_string(), self.timestamp());
        vars.insert("level".to_string(), 
                    record.metadata().level().to_string());
        vars.insert("file".to_string(), self.file(record));
        vars.insert("line".to_string(), self.line(record).to_string());
        vars.insert("args".to_string(), record.args().to_string());

        // Need to convert to std::fmt::Error
        match strfmt(self.msg_format, &vars) {
            Ok(m) => Ok(m),
            //Err(e) => std::fmt::Error::new(str(e)),
            // EYE - No access to strfmt::fmt::Error
            Err(e) => Ok(format!("Format error: {}", e)),
        }
    }

    fn timestamp(&self) -> String {
        match &self.timestamp_format {
            &"" => {
                "".to_string()
            },
            f => {
                let now = chrono::Local::now();
                now.format(&f).to_string()
            }
        }
    }

    fn line(&self, record: &log::Record) -> u32 {
        match record.line() {
            Some(l) => l,
            None => 0,
        }
    }

    fn file(&self, record: &log::Record) -> String {
        match record.file() {
            Some(f) => f.to_string(),
            None => "<no_file>".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//! Exit-code mapping and dialog results.

/// Dialog exit outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exit {
    /// User confirmed OK.
    Ok,
    /// User cancelled dialog.
    Cancel,
    /// Timeout elapsed while wait.
    Timeout,
    /// Internal error occurred.
    Err255,
}

impl Exit {
    /// Process exit code.
    #[must_use]
    pub const fn into_code(self) -> u8 {
        match self {
            Self::Ok => 0,
            Self::Cancel => 1,
            Self::Timeout => 5,
            Self::Err255 => 255,
        }
    }

    /// True only for OK.
    #[must_use]
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<Exit> for u8 {
    fn from(exit: Exit) -> Self {
        exit.into_code()
    }
}

impl std::fmt::Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::Cancel => write!(f, "cancel"),
            Self::Timeout => write!(f, "timeout"),
            Self::Err255 => write!(f, "error"),
        }
    }
}

impl std::error::Error for Exit {}

/// Dialog outcome with optional stdout data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogResult {
    /// Exit outcome of the dialog.
    pub exit: Exit,
    /// Stdout payload on OK, if any.
    pub output: Option<String>,
}

impl DialogResult {
    /// OK result with stdout data.
    #[must_use]
    pub fn ok(output: Option<String>) -> Self {
        Self {
            exit: Exit::Ok,
            output,
        }
    }

    /// Result without stdout data.
    #[must_use]
    pub fn bare(exit: Exit) -> Self {
        Self { exit, output: None }
    }

    /// Machine-readable result line for --json.
    #[must_use]
    pub fn to_json(&self) -> String {
        let value = match &self.output {
            Some(text) => format!("\"{}\"", escape_json(text)),
            None => "null".to_owned(),
        };
        format!("{{\"value\":{value},\"exit\":{}}}", self.exit.into_code())
    }
}

/// Escape text for JSON string.
fn escape_json(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_mapping_codes() {
        assert_eq!(Exit::Ok.into_code(), 0);
        assert_eq!(Exit::Cancel.into_code(), 1);
        assert_eq!(Exit::Timeout.into_code(), 5);
        assert_eq!(Exit::Err255.into_code(), 255);
    }

    #[test]
    fn exit_is_ok_only_for_ok() {
        assert!(Exit::Ok.is_ok());
        assert!(!Exit::Cancel.is_ok());
        assert!(!Exit::Timeout.is_ok());
        assert!(!Exit::Err255.is_ok());
    }

    #[test]
    fn result_json_escapes_value() {
        let result = DialogResult::ok(Some("a\"b\nc".to_owned()));
        assert_eq!(result.to_json(), r#"{"value":"a\"b\nc","exit":0}"#);
        assert_eq!(
            DialogResult::bare(Exit::Cancel).to_json(),
            r#"{"value":null,"exit":1}"#
        );
    }
}

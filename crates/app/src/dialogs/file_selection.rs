//! File-selection dialog.

use std::fs;
use std::path::{Path, PathBuf};

/// Default output separator.
pub const DEFAULT_SEPARATOR: &str = "\n";

/// File-selection config.
#[derive(Clone, Debug)]
pub struct FileSelection {
    /// Window title.
    pub title: String,
    /// Initial path.
    pub filename: Option<PathBuf>,
    /// Multiple selection?
    pub multiple: bool,
    /// Directory picker?
    pub directory: bool,
    /// Save mode?
    pub save: bool,
    /// Output separator.
    pub separator: Option<String>,
    /// Filter globs.
    pub file_filter: Vec<String>,
}

impl FileSelection {
    /// Create file dialog.
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            title,
            filename: None,
            multiple: false,
            directory: false,
            save: false,
            separator: None,
            file_filter: Vec::new(),
        }
    }

    /// Effective separator.
    #[must_use]
    pub fn effective_separator(&self) -> &str {
        self.separator
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(DEFAULT_SEPARATOR)
    }

    /// Start directory.
    #[must_use]
    pub fn start_dir(&self) -> PathBuf {
        match &self.filename {
            Some(path) => path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .map_or_else(|| PathBuf::from("."), Path::to_path_buf),
            None => PathBuf::from("."),
        }
    }

    /// Validate start directory.
    pub fn check_start_dir(&self) -> Result<(), String> {
        check_dir_readable(&self.start_dir())
    }

    /// Format picked paths.
    #[must_use]
    pub fn format_output(&self, paths: &[PathBuf]) -> String {
        let sep = self.effective_separator();
        paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(sep)
    }

    /// Path matches filter?
    #[must_use]
    pub fn matches_filter(&self, path: &Path) -> bool {
        if self.file_filter.is_empty() {
            return true;
        }
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.file_filter.iter().any(|pat| glob_match(pat, &name))
    }
}

/// Directory exists?
pub fn check_dir_readable(dir: &Path) -> Result<(), String> {
    let meta = fs::metadata(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if meta.permissions().mode() & 0o444 == 0 {
            return Err(format!("{}: permission denied", dir.display()));
        }
    }
    if !meta.is_dir() {
        return Err(format!("{}: not a directory", dir.display()));
    }
    fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    Ok(())
}

/// Minimal glob matcher.
#[must_use]
pub fn glob_match(pattern: &str, name: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(suffix) = pattern.strip_prefix("*.") {
        let dotted = format!(".{suffix}");
        return name.ends_with(&dotted) && name.len() > dotted.len();
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return name.starts_with(prefix);
    }
    pattern == name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glob_match_supports_star_and_suffix() {
        assert!(glob_match("*.txt", "notes.txt"));
        assert!(!glob_match("*.txt", "notes.md"));
        assert!(!glob_match("*.txt", "txt"));
        assert!(!glob_match("*.txt", "atxt"));
        assert!(!glob_match("*.txt", ".txt"));
        assert!(glob_match("foo*", "foobar"));
        assert!(glob_match("*", "anything"));
        assert!(glob_match("exact", "exact"));
        assert!(!glob_match("exact", "other"));
    }

    #[test]
    fn output_joins_with_separator() {
        let mut sel = FileSelection::new("T".to_owned());
        sel.separator = Some("|".to_owned());
        assert_eq!(
            sel.format_output(&[PathBuf::from("/a"), PathBuf::from("/b")]),
            "/a|/b"
        );
        let sel = FileSelection::new("T".to_owned());
        assert_eq!(sel.format_output(&[PathBuf::from("/a")]), "/a");
    }
}

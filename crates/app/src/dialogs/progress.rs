//! Progress dialog.

use std::io::{BufRead, IsTerminal};

use wicara_core::dialog::Exit;

/// Stdin buffer capacity.
pub const STDIN_CHANNEL_CAPACITY: usize = 64;

/// Stdin line effect.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressEvent {
    /// Label line.
    Text,
    /// Percentage line.
    Percentage,
    /// Reached 100%.
    Complete,
    /// Blank line.
    Ignored,
}

/// Progress bar model.
#[derive(Clone, Debug)]
pub struct ProgressState {
    /// Current percentage.
    pub percentage: f32,
    /// Label text.
    pub text: String,
}

impl ProgressState {
    /// Create progress state.
    #[must_use]
    pub fn new(initial_percentage: Option<u32>, text: String) -> Self {
        let percentage = initial_percentage
            .map(|p| p.clamp(0, 100) as f32)
            .unwrap_or(0.0);
        Self { percentage, text }
    }

    /// Bar complete?
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.percentage >= 100.0
    }

    /// Apply stdin line.
    pub fn apply_line(&mut self, line: &str) -> ProgressEvent {
        let line = line.trim();
        if line.is_empty() {
            return ProgressEvent::Ignored;
        }
        if let Some(rest) = line.strip_prefix('#') {
            self.text = rest.strip_prefix(' ').unwrap_or(rest).to_owned();
            return ProgressEvent::Text;
        }
        if let Some(value) = parse_percentage(line) {
            self.percentage = value.clamp(0.0, 100.0);
            if self.is_complete() {
                return ProgressEvent::Complete;
            }
            return ProgressEvent::Percentage;
        }
        ProgressEvent::Ignored
    }
}

fn parse_percentage(line: &str) -> Option<f32> {
    let digits = line.strip_suffix('%').unwrap_or(line).trim();
    digits.parse::<f32>().ok()
}

/// Progress dialog config.
#[derive(Clone, Debug)]
pub struct Progress {
    /// Window title.
    pub title: String,
    /// Label text.
    pub text: String,
    /// Starting percentage.
    pub percentage: Option<u32>,
    /// Pulsate mode?
    pub pulsate: bool,
    /// Auto-close?
    pub auto_close: bool,
    /// Auto-kill parent?
    pub auto_kill: bool,
    /// Hide cancel?
    pub no_cancel: bool,
    /// Show time remaining?
    pub time_remaining: bool,
}

impl Progress {
    /// Create progress dialog.
    #[must_use]
    pub fn new(title: String, text: String) -> Self {
        Self {
            title,
            text,
            percentage: None,
            pulsate: false,
            auto_close: false,
            auto_kill: false,
            no_cancel: false,
            time_remaining: false,
        }
    }

    /// Auto-close eligible?
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.pulsate
    }

    /// Run stdin loop.
    pub fn show_with_reader<R: BufRead + Send + 'static>(&self, reader: R) -> Exit {
        let mut state = ProgressState::new(self.percentage, self.text.clone());
        let mut rx = spawn_stdin_reader(reader);

        while let Some(line) = rx.blocking_recv() {
            let complete = matches!(state.apply_line(&line), ProgressEvent::Complete);
            if complete && self.auto_close && !self.pulsate {
                return Exit::Ok;
            }
        }

        Exit::Cancel
    }
}

/// Spawn stdin producer.
fn spawn_stdin_reader<R: BufRead + Send + 'static>(
    mut reader: R,
) -> tokio::sync::mpsc::Receiver<String> {
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(STDIN_CHANNEL_CAPACITY);
    std::thread::spawn(move || {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) | Err(_) => return,
                Ok(_) => {
                    let raw = line.trim_end_matches(['\n', '\r']).to_owned();
                    if tx.blocking_send(raw).is_err() {
                        return;
                    }
                }
            }
        }
    });
    rx
}

impl Progress {
    /// Run dialog, return outcome.
    pub fn run(&self) -> Exit {
        if !display_present() {
            if std::io::stdin().is_terminal() {
                return Exit::Cancel;
            }
            return self.show_with_reader(std::io::BufReader::new(std::io::stdin()));
        }
        let _ = crate::window::build_window(self.title.clone(), 400);
        self.show_with_reader(std::io::BufReader::new(std::io::stdin()))
    }
}

fn display_present() -> bool {
    !(std::env::var("WAYLAND_DISPLAY").is_err() && std::env::var("DISPLAY").is_err())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_lines_clamped_and_complete() {
        let mut state = ProgressState::new(None, "Working".to_owned());
        assert_eq!(state.apply_line("50"), ProgressEvent::Percentage);
        assert_eq!(state.percentage, 50.0);
        assert_eq!(state.apply_line("150"), ProgressEvent::Complete);
        assert_eq!(state.percentage, 100.0);
        assert!(state.is_complete());
    }

    #[test]
    fn hash_sets_text_percentage_suffix_ignored_lines() {
        let mut state = ProgressState::new(Some(10), "init".to_owned());
        assert_eq!(state.apply_line("# Downloading file"), ProgressEvent::Text);
        assert_eq!(state.text, "Downloading file");
        assert_eq!(state.apply_line("42%"), ProgressEvent::Percentage);
        assert_eq!(state.percentage, 42.0);
        assert_eq!(state.apply_line("not-a-number"), ProgressEvent::Ignored);
        assert_eq!(state.apply_line("   "), ProgressEvent::Ignored);
    }

    #[test]
    fn initial_percentage_clamped() {
        let state = ProgressState::new(Some(999), String::new());
        assert_eq!(state.percentage, 100.0);
    }
}

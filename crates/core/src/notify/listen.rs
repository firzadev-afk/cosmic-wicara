//! Listen stdin protocol.

use crate::notify::{APP_NAME, DEFAULT_ICON, DEFAULT_TIMEOUT_MS, NotificationRequest, NotifyError};
use std::io::BufRead;

/// Listen stdin command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListenCommand {
    /// Body text line.
    Text(String),
    /// Icon for next.
    Icon(String),
    /// Timeout for next.
    Timeout(i32),
    /// End stream.
    Close,
}

/// Parse one listen line.
#[must_use]
pub fn parse_listen_line(line: &str) -> Option<ListenCommand> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    if line.eq_ignore_ascii_case("close") {
        return Some(ListenCommand::Close);
    }
    if let Some(rest) = strip_command(line, "text") {
        return Some(ListenCommand::Text(rest.to_owned()));
    }
    if let Some(rest) = strip_command(line, "icon") {
        return Some(ListenCommand::Icon(rest.to_owned()));
    }
    if let Some(rest) = strip_command(line, "timeout") {
        return rest.parse::<i32>().ok().map(ListenCommand::Timeout);
    }
    Some(ListenCommand::Text(line.to_owned()))
}

fn strip_command<'a>(line: &'a str, command: &str) -> Option<&'a str> {
    let after = line.strip_prefix(command).and_then(|rest| {
        if let Some(stripped) = rest.strip_prefix('=') {
            Some(stripped.trim())
        } else if rest.starts_with(' ') {
            Some(rest.trim_start())
        } else {
            None
        }
    })?;
    Some(after.trim())
}

/// Collect listen notifications.
#[must_use]
pub fn collect_listen_requests(title: &str, input: &str) -> Vec<NotificationRequest> {
    let summary = if title.trim().is_empty() {
        APP_NAME.to_owned()
    } else {
        title.to_owned()
    };
    let mut icon = DEFAULT_ICON.to_owned();
    let mut timeout_ms = DEFAULT_TIMEOUT_MS;
    let mut out = Vec::new();
    for line in input.lines() {
        match parse_listen_line(line) {
            None => {}
            Some(ListenCommand::Close) => break,
            Some(ListenCommand::Icon(name)) => {
                if !name.is_empty() {
                    icon = name;
                }
            }
            Some(ListenCommand::Timeout(ms)) => timeout_ms = ms,
            Some(ListenCommand::Text(body)) => {
                if body.trim().is_empty() {
                    continue;
                }
                out.push(
                    NotificationRequest::new(summary.clone(), body)
                        .with_icon(icon.clone())
                        .with_timeout_ms(timeout_ms),
                );
            }
        }
    }
    out
}

/// Run listen, injected reader.
pub fn run_listen_with<R, F>(
    mut reader: R,
    title: &str,
    mut sender: F,
) -> Result<usize, NotifyError>
where
    R: BufRead,
    F: FnMut(&NotificationRequest) -> Result<u32, NotifyError>,
{
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .map_err(|err| NotifyError::Io(err.to_string()))?;
    let requests = collect_listen_requests(title, &input);
    let mut sent = 0usize;
    for request in &requests {
        sender(request)?;
        sent += 1;
    }
    Ok(sent)
}

/// Run listen from stdin.
pub fn run_listen_stdin(title: &str) -> Result<usize, NotifyError> {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut input = String::new();
    std::io::Read::read_to_string(&mut lock, &mut input)
        .map_err(|err| NotifyError::Io(err.to_string()))?;
    let requests = collect_listen_requests(title, &input);
    let mut sent = 0usize;
    for request in &requests {
        super::send_request(request)?;
        sent += 1;
    }
    Ok(sent)
}

//! Dialog geometry helpers.

/// Card padding pixels.
pub(crate) const CARD_PADDING: f32 = 16.0; // `space_m` padding on `dialog()`
/// Title text height.
pub(crate) const TITLE_HEIGHT: f32 = 26.0; // `text::title3` line
/// Gap between title and body.
pub(crate) const TITLE_GAP: f32 = 8.0; // `space_xxs` between title and body
/// Gap between content and buttons.
pub(crate) const BUTTON_GAP: f32 = 24.0; // `space_l` between content and button row
/// Button row height.
pub(crate) const BUTTON_ROW_HEIGHT: f32 = 40.0; // standard button height
/// Body line height.
pub(crate) const BODY_LINE_HEIGHT: f32 = 20.0; // `text::body` line
/// Body scroll cap.
pub(crate) const BODY_MAX_HEIGHT: f32 = 300.0; // `dialog()` body scroll cap
/// Body slack pixels.
pub(crate) const BODY_SLACK: f32 = 4.0;
/// Average glyph width estimate (conservative: underestimate capacity so the window never clips buttons).
pub(crate) const CHAR_WIDTH: f32 = 8.0;

/// Estimate wrapped lines.
pub(crate) fn wrapped_lines(text: &str, width: u32) -> usize {
    let usable = (width as f32 - CARD_PADDING * 2.0).max(CHAR_WIDTH * 10.0);
    let per_line = (usable / CHAR_WIDTH).max(1.0) as usize;
    text.lines()
        .map(|line| {
            let chars = line.chars().count();
            if chars == 0 {
                1
            } else {
                chars.div_ceil(per_line)
            }
        })
        .sum::<usize>()
        .max(1)
}

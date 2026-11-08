//! Color-selection dialog.
use wicara_core::dialog::Exit;

/// Color dialog state.
#[derive(Debug, Clone, Default)]
pub struct ColorSelection {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// Initial color seed.
    pub color: Option<String>,
    /// Show palette?
    pub show_palette: bool,
    /// Preferred width.
    pub width: u32,
}

impl ColorSelection {
    /// Create color dialog.
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: 400,
            ..Self::default()
        }
    }

    /// Set color seed.
    #[must_use]
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    /// Set show-palette.
    #[must_use]
    pub fn with_show_palette(mut self, show_palette: bool) -> Self {
        self.show_palette = show_palette;
        self
    }

    /// Parse hex color.
    pub fn parse(color: &str) -> Result<(u8, u8, u8), String> {
        let color = color.trim();
        if let Some(hex) = color.strip_prefix('#') {
            return Self::parse_hex(hex);
        }
        if let Some(inner) = color.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
            if parts.len() != 3 {
                return Err(format!("rgb() needs 3 channels: {color:?}"));
            }
            let mut ch = [0u8; 3];
            for (i, part) in parts.iter().enumerate() {
                let v: u16 = part
                    .parse()
                    .map_err(|_| format!("bad rgb channel {part:?} in {color:?}"))?;
                if v > 255 {
                    return Err(format!("rgb channel out of range in {color:?}"));
                }
                ch[i] = v as u8;
            }
            return Ok((ch[0], ch[1], ch[2]));
        }
        Err(format!("unsupported colour: {color:?}"))
    }

    fn parse_hex(hex: &str) -> Result<(u8, u8, u8), String> {
        let hex = hex.trim();
        if hex.len() == 3 {
            let mut ch = [0u8; 3];
            for (i, c) in hex.chars().enumerate() {
                let d = c
                    .to_digit(16)
                    .ok_or_else(|| format!("bad hex digit {c:?}"))? as u8;
                ch[i] = d * 16 + d;
            }
            Ok((ch[0], ch[1], ch[2]))
        } else if hex.len() == 6 {
            let mut ch = [0u8; 3];
            for (i, pair) in hex.as_bytes().chunks(2).enumerate() {
                let s = std::str::from_utf8(pair).map_err(|_| format!("bad hex pair in #{hex}"))?;
                ch[i] = u8::from_str_radix(s, 16)
                    .map_err(|_| format!("bad hex pair {s:?} in #{hex}"))?;
            }
            Ok((ch[0], ch[1], ch[2]))
        } else {
            Err(format!("hex must be #rgb or #rrggbb: #{hex}"))
        }
    }

    /// Format hex color.
    #[must_use]
    pub fn to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{r:02x}{g:02x}{b:02x}")
    }

    /// Canonical seed hex.
    pub fn canonical_output(&self) -> Result<Option<String>, Exit> {
        match &self.color {
            None => Ok(None),
            Some(seed) => Self::parse(seed)
                .map(|(r, g, b)| Some(Self::to_hex(r, g, b)))
                .map_err(|err| {
                    eprintln!("color-selection: {err}");
                    Exit::Err255
                }),
        }
    }

    /// Show palette?
    #[must_use]
    pub fn is_palette_visible(&self) -> bool {
        self.show_palette
    }

    /// Validate color seed.
    pub fn validate(&self) -> Result<(), Exit> {
        self.canonical_output().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forms() {
        assert_eq!(ColorSelection::parse("#0f0"), Ok((0, 255, 0)));
        assert_eq!(ColorSelection::parse("#ff0000"), Ok((255, 0, 0)));
        assert_eq!(ColorSelection::parse("rgb(1, 2, 3)"), Ok((1, 2, 3)));
        assert!(ColorSelection::parse("#12345").is_err());
        assert!(ColorSelection::parse("rgb(256, 0, 0)").is_err());
        assert_eq!(ColorSelection::to_hex(255, 0, 16), "#ff0010");
    }
}

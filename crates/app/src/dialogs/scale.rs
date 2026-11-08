//! Scale dialog.
use wicara_core::dialog::Exit;

/// Scale dialog state.
#[derive(Debug, Clone)]
pub struct Scale {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// Initial value.
    pub value: i64,
    /// Minimum value.
    pub min_value: i64,
    /// Maximum value.
    pub max_value: i64,
    /// Step size.
    pub step: i64,
    /// Print partials?
    pub print_partial: bool,
    /// Hide value?
    pub hide_value: bool,
    /// Preferred width.
    pub width: u32,
}

/// Scale defaults.
impl Default for Scale {
    fn default() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            value: 0,
            min_value: 0,
            max_value: 100,
            step: 1,
            print_partial: false,
            hide_value: false,
            width: 400,
        }
    }
}

impl Scale {
    /// Create scale dialog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set initial value.
    #[must_use]
    pub fn with_value(mut self, value: i64) -> Self {
        self.value = value;
        self
    }

    /// Set minimum value.
    #[must_use]
    pub fn with_min_value(mut self, min_value: i64) -> Self {
        self.min_value = min_value;
        self
    }

    /// Set maximum value.
    #[must_use]
    pub fn with_max_value(mut self, max_value: i64) -> Self {
        self.max_value = max_value;
        self
    }

    /// Set step size.
    #[must_use]
    pub fn with_step(mut self, step: i64) -> Self {
        self.step = step;
        self
    }

    /// Set print-partial.
    #[must_use]
    pub fn with_print_partial(mut self, print_partial: bool) -> Self {
        self.print_partial = print_partial;
        self
    }

    /// Set hide-value.
    #[must_use]
    pub fn with_hide_value(mut self, hide_value: bool) -> Self {
        self.hide_value = hide_value;
        self
    }

    /// Clamp to range.
    #[must_use]
    pub fn clamp(&self, v: i64) -> i64 {
        v.clamp(self.min_value, self.max_value)
    }

    /// Snap to step.
    #[must_use]
    pub fn snap(&self, v: i64) -> i64 {
        if self.step <= 1 {
            return v;
        }
        let offset = v - self.min_value;
        let snapped = (offset + self.step / 2).div_euclid(self.step) * self.step + self.min_value;
        self.clamp(snapped)
    }

    /// Snapped value.
    #[must_use]
    pub fn normalized_value(&self) -> i64 {
        self.snap(self.clamp(self.value))
    }

    /// Show value?
    #[must_use]
    pub fn is_value_visible(&self) -> bool {
        !self.hide_value
    }

    /// Format one value.
    #[must_use]
    pub fn format_value(value: i64) -> String {
        value.to_string()
    }

    /// Format partial values.
    #[must_use]
    pub fn partial_output(&self, values: &[i64]) -> Vec<String> {
        if !self.print_partial {
            return Vec::new();
        }
        values
            .iter()
            .map(|&v| Self::format_value(self.snap(self.clamp(v))))
            .collect()
    }

    /// Validate bounds.
    pub fn validate(&self) -> Result<(), Exit> {
        if self.min_value >= self.max_value {
            eprintln!(
                "scale: --min-value ({}) must be less than --max-value ({})",
                self.min_value, self.max_value
            );
            return Err(Exit::Err255);
        }
        if self.step <= 0 {
            eprintln!("scale: --step must be positive (got {})", self.step);
            return Err(Exit::Err255);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_wicara_defaults() {
        let s = Scale::new();
        assert_eq!(s.value, 0);
        assert_eq!((s.min_value, s.max_value), (0, 100));
        assert_eq!(s.step, 1);
    }

    #[test]
    fn clamp_and_snap() {
        let s = Scale::new()
            .with_min_value(0)
            .with_max_value(100)
            .with_step(5);
        assert_eq!(s.clamp(250), 100);
        assert_eq!(s.clamp(-10), 0);
        assert_eq!(s.snap(7), 5);
        assert_eq!(s.snap(8), 10);
        assert_eq!(s.snap(-3), 0);
    }

    #[test]
    fn partial_requires_flag() {
        assert!(Scale::new().partial_output(&[1, 2]).is_empty());
        let s = Scale::new().with_print_partial(true);
        assert_eq!(
            s.partial_output(&[3, 4]),
            vec!["3".to_owned(), "4".to_owned()]
        );
    }
}

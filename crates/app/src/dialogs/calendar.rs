//! Calendar dialog.
use wicara_core::dialog::Exit;

/// Default date format pattern.
pub const DEFAULT_DATE_FORMAT: &str = "%d/%m/%Y";

/// Calendar dialog state.
#[derive(Debug, Clone)]
pub struct Calendar {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// Day of month.
    pub day: u32,
    /// Month 1..=12.
    pub month: u32,
    /// Full year.
    pub year: u32,
    /// Date format pattern.
    pub date_format: Option<String>,
    /// Preferred width.
    pub width: u32,
}

impl Calendar {
    /// Create calendar dialog.
    #[must_use]
    pub fn new(day: u32, month: u32, year: u32) -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            day,
            month,
            year,
            date_format: None,
            width: 400,
        }
    }

    /// Set date format.
    #[must_use]
    pub fn with_date_format(mut self, fmt: String) -> Self {
        self.date_format = Some(fmt);
        self
    }

    /// Days per month.
    #[must_use]
    pub fn days_in_month(month: u32, year: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 0,
        }
    }

    /// Leap year check.
    #[must_use]
    pub fn is_leap_year(year: u32) -> bool {
        (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
    }

    /// Validate date.
    pub fn validate(&self) -> Result<(), Exit> {
        if !(1..=12).contains(&self.month) {
            eprintln!("calendar: --month out of range: {}", self.month);
            return Err(Exit::Err255);
        }
        let days = Self::days_in_month(self.month, self.year);
        if self.day == 0 || self.day > days {
            eprintln!(
                "calendar: --day out of range for {:04}-{:02}: {}",
                self.year, self.month, self.day
            );
            return Err(Exit::Err255);
        }
        Ok(())
    }

    /// Effective date pattern.
    #[must_use]
    pub fn pattern(&self) -> &str {
        self.date_format.as_deref().unwrap_or(DEFAULT_DATE_FORMAT)
    }

    /// Format date.
    #[must_use]
    pub fn format_date(&self) -> String {
        let pattern = self.pattern();
        Self::format_with(pattern, self.day, self.month, self.year)
    }

    /// Format with pattern.
    #[must_use]
    pub fn format_with(pattern: &str, day: u32, month: u32, year: u32) -> String {
        let mut out = String::new();
        let mut chars = pattern.chars();
        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.next() {
                    Some('Y') => out.push_str(&format!("{year:04}")),
                    Some('m') => out.push_str(&format!("{month:02}")),
                    Some('d') => out.push_str(&format!("{day:02}")),
                    Some('e') => out.push_str(&format!("{day:2}")),
                    Some('%') => out.push('%'),
                    Some(other) => {
                        out.push('%');
                        out.push(other);
                    }
                    None => out.push('%'),
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_fallback_and_escapes() {
        let cal = Calendar::new(8, 11, 2026);
        assert_eq!(cal.pattern(), DEFAULT_DATE_FORMAT);
        assert_eq!(cal.format_date(), "08/11/2026");
        assert_eq!(
            Calendar::format_with("%d/%m/%Y %%", 8, 11, 2026),
            "08/11/2026 %"
        );
        assert_eq!(Calendar::format_with("%Q", 8, 11, 2026), "%Q");
    }

    #[test]
    fn leap_rules() {
        assert!(Calendar::is_leap_year(2024));
        assert!(!Calendar::is_leap_year(2025));
        assert!(!Calendar::is_leap_year(1900));
        assert!(Calendar::is_leap_year(2000));
    }
}

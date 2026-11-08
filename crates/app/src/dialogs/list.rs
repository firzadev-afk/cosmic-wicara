//! List dialog.
use wicara_core::dialog::Exit;

/// Default column separator.
pub const DEFAULT_SEPARATOR: &str = "|";

/// Row selector mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// Plain rows.
    #[default]
    Plain,
    /// Checklist rows.
    Checklist,
    /// Radio rows.
    Radiolist,
    /// Image rows.
    Imagelist,
}

/// List dialog state.
#[derive(Debug, Clone)]
pub struct List {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// Column headers.
    pub columns: Vec<String>,
    /// Flat row cells.
    pub rows: Vec<String>,
    /// Selector mode.
    pub mode: SelectionMode,
    /// Editable cells?
    pub editable: bool,
    /// Multiple selection?
    pub multiple: bool,
    /// Print-column spec.
    pub print_column: Option<String>,
    /// Hidden columns.
    pub hide_column: Vec<String>,
    /// Hide header?
    pub hide_header: bool,
    /// Column separator.
    pub separator: String,
    /// Preferred width.
    pub width: u32,
}

impl Default for List {
    fn default() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            mode: SelectionMode::Plain,
            editable: false,
            multiple: false,
            print_column: None,
            hide_column: Vec::new(),
            hide_header: false,
            separator: DEFAULT_SEPARATOR.to_owned(),
            width: 400,
        }
    }
}

impl List {
    /// Create list dialog.
    #[must_use]
    pub fn new(columns: Vec<String>, rows: Vec<String>) -> Self {
        Self {
            columns,
            rows,
            ..Self::default()
        }
    }

    /// Chainable title setter.
    #[must_use]
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Set body text.
    #[must_use]
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    /// Set checklist mode.
    #[must_use]
    pub fn with_checklist(mut self, checklist: bool) -> Self {
        self.mode = if checklist {
            SelectionMode::Checklist
        } else {
            SelectionMode::Plain
        };
        self
    }

    /// Set radio mode.
    #[must_use]
    pub fn with_radiolist(mut self, radiolist: bool) -> Self {
        self.mode = if radiolist {
            SelectionMode::Radiolist
        } else {
            SelectionMode::Plain
        };
        self
    }

    /// Set image mode.
    #[must_use]
    pub fn with_imagelist(mut self, imagelist: bool) -> Self {
        self.mode = if imagelist {
            SelectionMode::Imagelist
        } else {
            SelectionMode::Plain
        };
        self
    }

    /// Set editable cells.
    #[must_use]
    pub fn with_editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Chainable print-column setter.
    #[must_use]
    pub fn with_print_column(mut self, spec: Option<String>) -> Self {
        self.print_column = spec;
        self
    }

    /// Set hidden columns.
    #[must_use]
    pub fn with_hide_column(mut self, hide: Vec<String>) -> Self {
        self.hide_column = hide;
        self
    }

    /// Set hide-header.
    #[must_use]
    pub fn with_hide_header(mut self, hide: bool) -> Self {
        self.hide_header = hide;
        self
    }

    /// Chainable separator setter.
    #[must_use]
    pub fn with_separator(mut self, separator: String) -> Self {
        self.separator = separator;
        self
    }

    /// Checklist mode?
    #[must_use]
    pub fn is_checklist(&self) -> bool {
        self.mode == SelectionMode::Checklist
    }

    /// Radio mode?
    #[must_use]
    pub fn is_radiolist(&self) -> bool {
        self.mode == SelectionMode::Radiolist
    }

    /// Image mode?
    #[must_use]
    pub fn is_imagelist(&self) -> bool {
        self.mode == SelectionMode::Imagelist
    }

    /// Column count.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Row count.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows
            .len()
            .checked_div(self.column_count())
            .unwrap_or(0)
    }

    /// Row slice.
    #[must_use]
    pub fn row(&self, row_idx: usize) -> Option<&[String]> {
        let n = self.column_count();
        self.rows.get(row_idx * n..(row_idx + 1) * n)
    }

    /// Parse column spec.
    fn parse_column_token(token: &str, ncols: usize) -> Result<Vec<usize>, Exit> {
        let token = token.trim();
        if token.is_empty() {
            return Err(Exit::Err255);
        }
        if let Some((lo, hi)) = token.split_once('-') {
            let lo: usize = lo.trim().parse().map_err(|_| Exit::Err255)?;
            let hi: usize = hi.trim().parse().map_err(|_| Exit::Err255)?;
            if lo == 0 || hi == 0 || lo > hi || hi > ncols {
                return Err(Exit::Err255);
            }
            Ok((lo - 1..hi).collect())
        } else {
            let n: usize = token.parse().map_err(|_| Exit::Err255)?;
            if n == 0 || n > ncols {
                Err(Exit::Err255)
            } else {
                Ok(vec![n - 1])
            }
        }
    }

    /// Parse print-column spec.
    pub fn parse_print_column(spec: &str, ncols: usize) -> Result<Vec<usize>, Exit> {
        if ncols == 0 {
            return Err(Exit::Err255);
        }
        let spec = spec.trim();
        if spec.is_empty() {
            return Err(Exit::Err255);
        }
        if spec.eq_ignore_ascii_case("all") {
            return Ok((0..ncols).collect());
        }
        let mut out = Vec::new();
        for token in spec.split(',') {
            let token = token.trim();
            if token.eq_ignore_ascii_case("all") {
                return Ok((0..ncols).collect());
            }
            out.extend(Self::parse_column_token(token, ncols)?);
        }
        if out.is_empty() {
            Err(Exit::Err255)
        } else {
            Ok(out)
        }
    }

    /// Columns to print.
    fn print_columns(&self) -> Result<Vec<usize>, Exit> {
        let n = self.column_count();
        if n == 0 {
            return Err(Exit::Err255);
        }
        let wanted = match &self.print_column {
            Some(spec) => Self::parse_print_column(spec, n)?,
            None => (0..n).collect(),
        };
        let hidden = self.hidden_indices()?;
        Ok(wanted.into_iter().filter(|i| !hidden.contains(i)).collect())
    }

    /// Hidden column indices.
    fn hidden_indices(&self) -> Result<Vec<usize>, Exit> {
        let n = self.column_count();
        let mut hidden = Vec::new();
        for spec in &self.hide_column {
            for token in spec.split(',') {
                let token = token.trim();
                if token.is_empty() {
                    continue;
                }
                hidden.extend(Self::parse_column_token(token, n)?);
            }
        }
        Ok(hidden)
    }

    /// Visible column indices.
    #[must_use]
    pub fn visible_columns(&self) -> Vec<usize> {
        let n = self.column_count();
        let Ok(hidden) = self.hidden_indices() else {
            return (0..n).collect();
        };
        (0..n).filter(|i| !hidden.contains(i)).collect()
    }

    /// Validate list specs.
    pub fn validate(&self) -> Result<(), Exit> {
        let n = self.column_count();
        if n == 0 {
            eprintln!("list: at least one --column is required");
            return Err(Exit::Err255);
        }
        if !self.rows.len().is_multiple_of(n) {
            eprintln!(
                "list: row data length {} is not a multiple of {} column(s)",
                self.rows.len(),
                n
            );
            return Err(Exit::Err255);
        }
        if let Some(spec) = &self.print_column {
            Self::parse_print_column(spec, n)?;
        }
        self.hidden_indices()?;
        self.print_columns()?;
        Ok(())
    }

    /// Serialize selected rows.
    pub fn selected_output(&self, selected_rows: &[usize]) -> Result<Vec<String>, Exit> {
        self.validate()?;
        let cols = self.print_columns()?;
        let mut lines = Vec::with_capacity(selected_rows.len());
        for &row_idx in selected_rows {
            let row = self.row(row_idx).ok_or(Exit::Err255)?;
            let cells: Vec<&str> = cols.iter().map(|&c| row[c].as_str()).collect();
            lines.push(cells.join(&self.separator));
        }
        Ok(lines)
    }

    /// Header row.
    #[must_use]
    pub fn header_line(&self) -> String {
        if self.hide_header {
            String::new()
        } else {
            self.columns.join(&self.separator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> List {
        List::new(
            vec!["ID".to_owned(), "Name".to_owned(), "City".to_owned()],
            vec![
                "1".to_owned(),
                "Andi".to_owned(),
                "Bandung".to_owned(),
                "2".to_owned(),
                "Budi".to_owned(),
                "Jakarta".to_owned(),
            ],
        )
    }

    #[test]
    fn print_column_parse_forms() {
        assert_eq!(List::parse_print_column("ALL", 3), Ok(vec![0, 1, 2]));
        assert_eq!(List::parse_print_column("all", 3), Ok(vec![0, 1, 2]));
        assert_eq!(List::parse_print_column("2", 3), Ok(vec![1]));
        assert_eq!(List::parse_print_column("3,1", 3), Ok(vec![2, 0]));
        assert_eq!(List::parse_print_column("1-2", 3), Ok(vec![0, 1]));
        assert_eq!(List::parse_print_column("9", 3), Err(Exit::Err255));
        assert_eq!(List::parse_print_column("0", 3), Err(Exit::Err255));
    }

    #[test]
    fn selected_output_defaults_to_all_columns() {
        assert_eq!(
            sample().selected_output(&[0]),
            Ok(vec!["1|Andi|Bandung".to_owned()])
        );
    }
}

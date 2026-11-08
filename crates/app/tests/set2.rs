//! List print-column tests.

use app::dialogs::{
    calendar::Calendar, color_selection::ColorSelection, list::List, scale::Scale,
    text_info::TextInfo,
};
use wicara_core::dialog::Exit;

fn sample_list() -> List {
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
fn print_column_single_column() {
    let mut list = sample_list();
    list.print_column = Some("2".to_owned());
    assert_eq!(
        list.selected_output(&[0, 1]),
        Ok(vec!["Andi".to_owned(), "Budi".to_owned()])
    );
}

#[test]
fn print_column_all_uses_separator() {
    let mut list = sample_list();
    assert_eq!(list.separator, "|");
    list.print_column = Some("ALL".to_owned());
    assert_eq!(
        list.selected_output(&[0]),
        Ok(vec!["1|Andi|Bandung".to_owned()])
    );
}

#[test]
fn print_column_default_prints_all_columns() {
    let list = sample_list();
    assert_eq!(
        list.selected_output(&[1]),
        Ok(vec!["2|Budi|Jakarta".to_owned()])
    );
}

#[test]
fn print_column_multiple_preserves_given_order() {
    let mut list = sample_list();
    list.print_column = Some("3,1".to_owned());
    assert_eq!(list.selected_output(&[0]), Ok(vec!["Bandung|1".to_owned()]));
}

#[test]
fn print_column_range_and_custom_separator() {
    let mut list = sample_list();
    list.print_column = Some("1-2".to_owned());
    list.separator = ",".to_owned();
    assert_eq!(
        list.selected_output(&[0, 1]),
        Ok(vec!["1,Andi".to_owned(), "2,Budi".to_owned()])
    );
}

#[test]
fn print_column_rejects_invalid_column() {
    let mut list = sample_list();
    list.print_column = Some("9".to_owned());
    assert_eq!(list.selected_output(&[0]), Err(Exit::Err255));
    list.print_column = Some("0".to_owned());
    assert_eq!(list.selected_output(&[0]), Err(Exit::Err255));
}

#[test]
fn list_hide_column_excludes_from_visible_and_output() {
    let mut list = sample_list();
    list.hide_column = vec!["2".to_owned()];
    assert_eq!(list.visible_columns(), vec![0usize, 2usize]);
    assert_eq!(list.selected_output(&[0]), Ok(vec!["1|Bandung".to_owned()]));
    assert_eq!(list.validate(), Ok(()));
}

#[test]
fn list_rejects_mismatched_row_width() {
    let list = List::new(
        vec!["A".to_owned(), "B".to_owned()],
        vec!["only".to_owned()],
    );
    assert_eq!(list.validate(), Err(Exit::Err255));
    assert_eq!(list.selected_output(&[0]), Err(Exit::Err255));
}

#[test]
fn scale_clamps_snaps_and_validates_bounds() {
    let mut scale = Scale::new();
    scale.value = 250;
    scale.min_value = 0;
    scale.max_value = 100;
    scale.step = 5;
    assert_eq!(scale.normalized_value(), 100);
    assert_eq!(scale.validate(), Ok(()));
    assert_eq!(Scale::format_value(42), "42");
    assert!(scale.is_value_visible());

    let mut bad = Scale::new();
    bad.min_value = 100;
    bad.max_value = 0;
    assert_eq!(bad.validate(), Err(Exit::Err255));

    let mut bad_step = Scale::new();
    bad_step.step = 0;
    assert_eq!(bad_step.validate(), Err(Exit::Err255));
}

#[test]
fn calendar_formats_and_rejects_bad_dates() {
    let mut cal = Calendar::new(8, 11, 2026);
    cal.date_format = Some("%Y-%m-%d".to_owned());
    assert_eq!(cal.format_date(), "2026-11-08");
    let default = Calendar::new(8, 11, 2026);
    assert_eq!(default.format_date(), "08/11/2026");
    assert_eq!(cal.validate(), Ok(()));
    assert_eq!(Calendar::new(31, 2, 2026).validate(), Err(Exit::Err255));
    assert_eq!(Calendar::new(29, 2, 2024).validate(), Ok(())); // leap year
    assert_eq!(Calendar::new(29, 2, 2025).validate(), Err(Exit::Err255));
}

#[test]
fn color_canonicalizes_to_hex() {
    let mut color = ColorSelection::new();
    color.color = Some("#FF0000".to_owned());
    assert_eq!(color.canonical_output(), Ok(Some("#ff0000".to_owned())));
    color.color = Some("rgb(0, 255, 0)".to_owned());
    assert_eq!(color.canonical_output(), Ok(Some("#00ff00".to_owned())));
    color.color = Some("not-a-color".to_owned());
    assert_eq!(color.canonical_output(), Err(Exit::Err255));
    assert_eq!(color.validate(), Err(Exit::Err255));
}

#[test]
fn text_info_checkbox_contract() {
    let mut info = TextInfo::new();
    info.checkbox = Some("I have read and agree".to_owned());
    assert!(info.checkbox_required());
    assert!(!info.checkbox_satisfied(false));
    assert!(info.checkbox_satisfied(true));
    let plain = TextInfo::new();
    assert!(!plain.checkbox_required());
    assert!(plain.checkbox_satisfied(false));
    assert_eq!(plain.resolve_text(), Ok(String::new()));
}

//! Calendar dialog builder.

use wicara_app::dialogs::calendar::Calendar;
use wicara_app::runner::DialogRequest;

use super::today;
use cli::args::Cli;

pub fn build(cli: &Cli, title: String, text: String, width: u32) -> Result<DialogRequest, ()> {
    let (today_day, today_month, today_year) = today();
    let mut cal = Calendar::new(
        cli.per_dialog.day.unwrap_or(today_day),
        cli.per_dialog.month.unwrap_or(today_month),
        cli.per_dialog.year.unwrap_or(today_year),
    );
    cal.title = title;
    cal.text = text;
    cal.width = width;
    if let Some(format) = &cli.per_dialog.date_format {
        cal.date_format = Some(format.clone());
    }
    if cal.validate().is_err() {
        return Err(());
    }
    Ok(DialogRequest::Calendar(cal))
}

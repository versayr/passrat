use chrono::{Datelike, NaiveDate};
use ordinal::ToOrdinal;
use ratatui::text::Line;

pub fn format_current_date(date: NaiveDate) -> String {
    format!(
        "{}, {} {}, {}",
        date.format("%A"),
        date.format("%B"),
        date.day().to_ordinal_string(),
        date.format("%Y")
    )
}

pub fn construct_detail_field(label: &str, value: &str, width: usize) -> Line<'static> {
    Line::from(vec![
        format!("{label: <width$}").into(),
        value.to_string().into(),
    ])
}

use chrono::{Datelike, Local};
use ordinal::ToOrdinal;

pub fn format_current_date() -> String {
    let now = Local::now();
    format!(
        "{}, {} {}, {}",
        now.format("%A"),
        now.format("%B"),
        now.day().to_ordinal_string(),
        now.format("%Y")
    )
}

use crate::rules::OnHours;
use std::collections::HashMap;

pub fn format_on_hours(on_hours: &HashMap<String, OnHours>) -> String {
    let mut report = String::new();

    for (device, on_hours) in on_hours {
        report.push_str(&format!("  · {}:", device));

        for hour in 0..24 {
            if on_hours.on_at(hour) {
                report.push_str(&format!(" {}:00 ", hour));
            }
        }

        report.push_str("\n");
    }

    report
}

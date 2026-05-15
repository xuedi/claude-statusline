use chrono::{DateTime, Local, TimeZone};
use serde_json::Value;

pub fn epoch_from_value(v: Option<&Value>) -> Option<i64> {
    v?.as_i64().filter(|&n| n > 0)
}

pub fn iso_to_epoch(iso: &str) -> Option<i64> {
    DateTime::parse_from_rfc3339(iso)
        .ok()
        .map(|dt| dt.timestamp())
}

// WHY: an unparseable epoch is unreachable in practice (epochs come from the API).
// Falling back to `Local::now` is wrong in theory but never observed in the wild;
// keeping the behaviour avoids surfacing a panic for a case that does not occur.
pub fn epoch_to_local(epoch: i64) -> DateTime<Local> {
    Local
        .timestamp_opt(epoch, 0)
        .single()
        .unwrap_or_else(Local::now)
}

pub fn format_hhmm(epoch: i64) -> String {
    epoch_to_local(epoch).format("%H:%M").to_string()
}

pub fn format_datetime(epoch: i64) -> String {
    epoch_to_local(epoch).format("%b %-d, %H:%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn iso_to_epoch_matrix() {
        let cases: &[(&str, Option<i64>)] = &[
            // --- valid ---
            ("2026-05-14T12:00:00+00:00", Some(1778760000)),
            ("2026-05-14T12:00:00Z", Some(1778760000)), // Z shorthand
            ("2026-05-14T14:00:00+02:00", Some(1778760000)), // same instant, CET offset
            ("1970-01-01T00:00:00Z", Some(0)),          // unix epoch zero
            // --- invalid ---
            ("not a date", None),
            ("", None),
            ("2026-05-14", None),          // date only, no time - not RFC 3339
            ("2026-05-14T12:00:00", None), // missing timezone
        ];
        for &(input, expected) in cases {
            assert_eq!(iso_to_epoch(input), expected, "iso_to_epoch({input:?})");
        }
    }

    #[test]
    fn epoch_from_value_matrix() {
        let cases: &[(Option<Value>, Option<i64>)] = &[
            // --- accepted ---
            (Some(json!(1234567890)), Some(1234567890)),
            (Some(json!(1)), Some(1)),
            // --- rejected ---
            (Some(json!(0)), None),
            (Some(json!(-1)), None),
            (Some(json!(-999)), None),
            (None, None),
            // --- wrong JSON types ---
            (Some(json!(1.5)), None),
            (Some(json!("1234567890")), None),
            (Some(json!(true)), None),
            (Some(json!(null)), None),
        ];
        for (input, expected) in cases {
            assert_eq!(
                epoch_from_value(input.as_ref()),
                *expected,
                "epoch_from_value({input:?})"
            );
        }
    }
}

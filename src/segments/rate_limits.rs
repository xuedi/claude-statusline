use serde_json::Value;

use crate::api;
use crate::cache;
use crate::input::{Input, RateLimit, RateLimits};
use crate::time::{epoch_from_value, format_datetime, format_hhmm, iso_to_epoch};

pub fn render(input: &Input) -> Vec<String> {
    if is_effective_builtin(input.rate_limits.as_ref()) {
        return builtin_segments(input.rate_limits.as_ref().unwrap());
    }

    let cache_file = cache::usage_cache_file();
    let cached = cache::load_json_cache(&cache_file);

    let data = if cache::cache_is_fresh(&cache_file) {
        cached
    } else {
        api::fetch_usage(&cache_file).ok().or(cached)
    };

    match data {
        Some(d) => api_segments(&d),
        None => vec!["5h -".to_string(), "7d -".to_string()],
    }
}

fn is_effective_builtin(rl: Option<&RateLimits>) -> bool {
    let Some(rl) = rl else {
        return false;
    };
    if has_nonzero_pct(rl.five_hour.as_ref()) || has_nonzero_pct(rl.seven_day.as_ref()) {
        return true;
    }
    has_reset_timestamp(rl.five_hour.as_ref()) || has_reset_timestamp(rl.seven_day.as_ref())
}

fn has_nonzero_pct(rl: Option<&RateLimit>) -> bool {
    rl.and_then(|r| r.used_percentage)
        .map(|p| p != 0.0)
        .unwrap_or(false)
}

fn has_reset_timestamp(rl: Option<&RateLimit>) -> bool {
    rl.and_then(|r| r.resets_at.as_ref())
        .and_then(|v| v.as_i64())
        .map(|n| n > 0)
        .unwrap_or(false)
}

fn builtin_segments(rl: &RateLimits) -> Vec<String> {
    let mut segs = Vec::new();

    if let Some(five) = &rl.five_hour {
        if let Some(pct) = five.used_percentage {
            let mut seg = format!("5h {}%", pct.round() as i64);
            if let Some(epoch) = epoch_from_value(five.resets_at.as_ref()) {
                seg.push_str(&format!(" @{}", format_hhmm(epoch)));
            }
            segs.push(seg);
        }
    }

    if let Some(seven) = &rl.seven_day {
        if let Some(pct) = seven.used_percentage {
            let mut seg = format!("7d {}%", pct.round() as i64);
            if let Some(epoch) = epoch_from_value(seven.resets_at.as_ref()) {
                seg.push_str(&format!(" @{}", format_datetime(epoch)));
            }
            segs.push(seg);
        }
    }

    segs
}

fn api_segments(data: &Value) -> Vec<String> {
    let mut segs = Vec::new();

    if let Some(five) = data.get("five_hour") {
        segs.push(api_segment(five, "5h", true));
    }
    if let Some(seven) = data.get("seven_day") {
        segs.push(api_segment(seven, "7d", false));
    }
    segs
}

fn api_segment(window: &Value, label: &str, hhmm: bool) -> String {
    let pct = window
        .get("utilization")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let mut seg = format!("{label} {}%", pct.round() as i64);
    if let Some(iso) = window.get("resets_at").and_then(|v| v.as_str()) {
        if let Some(epoch) = iso_to_epoch(iso) {
            let stamp = if hhmm {
                format_hhmm(epoch)
            } else {
                format_datetime(epoch)
            };
            seg.push_str(&format!(" @{stamp}"));
        }
    }
    seg
}

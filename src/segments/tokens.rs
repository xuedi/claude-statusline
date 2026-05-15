use crate::bar;
use crate::input::Input;

pub fn render(input: &Input) -> Option<String> {
    let cw = input.context_window.as_ref()?;
    let size = cw.context_window_size;
    if size == 0 {
        return None;
    }
    let used = cw.current_usage.input_tokens
        + cw.current_usage.cache_creation_input_tokens
        + cw.current_usage.cache_read_input_tokens;
    let pct = used as f64 * 100.0 / size as f64;
    Some(format!(
        "{}/{} [{}]",
        format_tokens(used),
        format_tokens(size),
        bar::render(pct)
    ))
}

fn format_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        let v = n as f64 / 1_000_000.0;
        let rounded = (v * 10.0).round() / 10.0;
        if is_whole(rounded) {
            format!("{}m", rounded as u64)
        } else {
            format!("{rounded:.1}m")
        }
    } else if n >= 1_000 {
        format!("{}k", (n as f64 / 1_000.0).round() as u64)
    } else {
        format!("{n}")
    }
}

fn is_whole(x: f64) -> bool {
    (x - x.floor()).abs() < f64::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_tokens_matrix() {
        let cases: &[(u64, &str)] = &[
            // --- raw (below 1k) ---
            (0, "0"),
            (1, "1"),
            (500, "500"),
            (999, "999"),
            // --- k range ---
            (1_000, "1k"),
            (1_499, "1k"),
            (1_500, "2k"),
            (10_000, "10k"),
            (100_000, "100k"),
            (999_499, "999k"),
            (999_500, "1000k"), // no auto-promotion to "1m"
            (999_999, "1000k"),
            // --- m range ---
            (1_000_000, "1m"),
            (1_049_999, "1m"),   // rounds down: 1.049999 * 10 = 10.4999 -> 10
            (1_050_000, "1.1m"), // rounds up:   1.05 * 10 = 10.5 -> 11
            (1_500_000, "1.5m"),
            (2_000_000, "2m"),
            (9_999_999, "10m"), // 9.999999 rounds to whole 10
            (10_000_000, "10m"),
        ];
        for &(input, expected) in cases {
            assert_eq!(format_tokens(input), expected, "format_tokens({input})");
        }
    }
}

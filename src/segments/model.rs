use crate::input::Input;

pub fn render(input: &Input) -> String {
    input
        .model
        .as_ref()
        .and_then(|m| m.display_name.as_deref())
        .map(format_model)
        .unwrap_or_else(|| "Claude".to_string())
}

fn format_model(name: &str) -> String {
    let Some(paren) = name.find(" (") else {
        return name.to_string();
    };
    let rest = &name[paren + 2..];
    let Some(ctx_end) = rest.find(" context)") else {
        return name.to_string();
    };
    let ctx_val = &rest[..ctx_end];
    let base = &name[..paren];
    format!("{base} {ctx_val}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_model_matrix() {
        let cases: &[(&str, &str)] = &[
            // --- standard context-paren stripping ---
            ("Claude Sonnet 4.6 (1M context)", "Claude Sonnet 4.6 1M"),
            ("Claude Opus 4.6 (1M context)", "Claude Opus 4.6 1M"),
            ("Claude Haiku 4.5 (200K context)", "Claude Haiku 4.5 200K"),
            // --- no parens at all ---
            ("Claude Opus 4.7", "Claude Opus 4.7"),
            ("", ""),
            // --- parens without " context)" - left untouched ---
            ("Claude (beta)", "Claude (beta)"),
            ("Claude (fast mode)", "Claude (fast mode)"),
            // --- trailing text after context paren is dropped ---
            ("Claude (1M context) beta", "Claude 1M"),
            // --- paren at the very start: empty base still gets a leading space ---
            (" (512K context)", " 512K"),
        ];
        for &(input, expected) in cases {
            assert_eq!(format_model(input), expected, "format_model({input:?})");
        }
    }
}

use claude_statusline::render;
use pretty_assertions::assert_eq;

#[test]
fn empty_input_renders_claude_literal() {
    assert_eq!(render(""), "Claude");
    assert_eq!(render("   \n"), "Claude");
}

#[test]
fn malformed_json_does_not_panic() {
    let out = render("{not valid json");
    assert!(out.starts_with("Claude"), "got: {out}");
    assert!(out.contains("Effort:"));
}

#[test]
fn full_builtin_payload_emits_expected_segments() {
    let payload = r#"{
        "model": {"display_name": "Claude Sonnet 4.6 (1M context)"},
        "cwd": "/nonexistent-not-a-git-repo-12345",
        "context_window": {
            "context_window_size": 1000000,
            "current_usage": {"input_tokens": 250000}
        },
        "rate_limits": {
            "five_hour":  {"used_percentage": 42.0, "resets_at": 0},
            "seven_day":  {"used_percentage": 18.0, "resets_at": 0}
        }
    }"#;

    let out = render(payload);
    let parts: Vec<&str> = out.split(" | ").collect();

    assert_eq!(parts[0], "Claude Sonnet 4.6 1M");
    // effort comes right after model
    assert_eq!(
        parts[1],
        format!("Effort: {}", parts[1].trim_start_matches("Effort: "))
    );
    // git segment is absent (cwd is not a repo) so tokens comes next
    assert!(parts[2].starts_with("250k/1m ["));
    assert_eq!(parts[3], "HourlyReset: 42%");
    assert_eq!(parts[4], "WeeklyReset: 18%");
}

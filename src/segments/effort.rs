use std::env;
use std::fs;

use serde_json::Value;

// WHY: settings.json wins when present; the env var is only the fallback for sessions
// that have not customized the file. This order is the documented contract - flipping
// it would silently override per-user settings any time the env var leaks in.
pub fn render() -> String {
    if let Some(level) = effort_from_settings() {
        return normalize(&level);
    }
    env::var("CLAUDE_CODE_EFFORT_LEVEL").unwrap_or_else(|_| "med".to_string())
}

fn effort_from_settings() -> Option<String> {
    let home = env::var("HOME").ok()?;
    let path = format!("{home}/.claude/settings.json");
    let content = fs::read_to_string(&path).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;
    v.get("effortLevel")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

fn normalize(level: &str) -> String {
    match level {
        "medium" => "med".to_string(),
        other => other.to_string(),
    }
}

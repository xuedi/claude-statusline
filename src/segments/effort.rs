use std::env;
use std::fs;

use serde_json::Value;

use crate::input::Input;

// WHY: the stdin payload carries the live per-session effort under `effort.level`, so it
// wins outright. settings.json is only a static default and goes stale the moment the
// session changes effort; it and the env var are fallbacks for payloads that omit the
// field (e.g. models without an effort parameter).
pub fn render(input: &Input) -> String {
    if let Some(level) = input
        .effort
        .as_ref()
        .and_then(|e| e.level.as_deref())
        .filter(|s| !s.is_empty())
    {
        return normalize(level);
    }
    if let Some(level) = effort_from_settings() {
        return normalize(&level);
    }
    env::var("CLAUDE_EFFORT")
        .or_else(|_| env::var("CLAUDE_CODE_EFFORT_LEVEL"))
        .unwrap_or_else(|_| "med".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Effort;

    fn input_with(level: Option<&str>) -> Input {
        Input {
            effort: level.map(|l| Effort {
                level: Some(l.to_string()),
            }),
            ..Default::default()
        }
    }

    #[test]
    fn payload_effort_wins_and_normalizes_medium() {
        assert_eq!(render(&input_with(Some("medium"))), "med");
    }

    #[test]
    fn payload_effort_passes_through_higher_levels() {
        assert_eq!(render(&input_with(Some("xhigh"))), "xhigh");
        assert_eq!(render(&input_with(Some("max"))), "max");
    }

    #[test]
    fn empty_payload_level_falls_through() {
        assert_ne!(render(&input_with(Some(""))), "");
    }
}

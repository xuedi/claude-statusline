use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};

use serde_json::Value;

pub const CACHE_TTL: Duration = Duration::from_secs(60);
pub const CACHE_DIR: &str = "/tmp/claude";

pub fn usage_cache_file() -> String {
    let home = env::var("HOME").unwrap_or_default();
    let config_dir = env::var("CLAUDE_CONFIG_DIR").unwrap_or_else(|_| format!("{home}/.claude"));
    let hash = short_hash(&config_dir);
    format!("{CACHE_DIR}/statusline-usage-cache-{hash}.json")
}

// WHY: the hash only disambiguates cache files across CLAUDE_CONFIG_DIR values.
// It is not security-sensitive, so DefaultHasher is fine even though it is not stable.
pub fn short_hash(s: &str) -> String {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:016x}", h.finish())[..8].to_string()
}

pub fn cache_is_fresh(path: &str) -> bool {
    let Ok(meta) = fs::metadata(path) else {
        return false;
    };
    let Ok(mtime) = meta.modified() else {
        return false;
    };
    SystemTime::now()
        .duration_since(mtime)
        .map(|age| age < CACHE_TTL)
        .unwrap_or(false)
}

pub fn load_json_cache(path: &str) -> Option<Value> {
    let content = fs::read_to_string(path).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;
    v.get("five_hour").is_some().then_some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_hash_is_deterministic() {
        assert_eq!(short_hash("foo"), short_hash("foo"));
    }

    #[test]
    fn short_hash_differs_for_different_inputs() {
        assert_ne!(short_hash("foo"), short_hash("bar"));
    }

    #[test]
    fn short_hash_is_eight_hex_chars() {
        let h = short_hash("anything");
        assert_eq!(h.len(), 8);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

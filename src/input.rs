use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Input {
    pub model: Option<Model>,
    pub cwd: Option<String>,
    pub context_window: Option<ContextWindow>,
    pub rate_limits: Option<RateLimits>,
    pub effort: Option<Effort>,
}

#[derive(Debug, Deserialize)]
pub struct Effort {
    pub level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ContextWindow {
    pub context_window_size: u64,
    pub current_usage: TokenUsage,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

#[derive(Debug, Deserialize)]
pub struct RateLimits {
    pub five_hour: Option<RateLimit>,
    pub seven_day: Option<RateLimit>,
}

#[derive(Debug, Deserialize)]
pub struct RateLimit {
    pub used_percentage: Option<f64>,
    pub resets_at: Option<Value>,
}

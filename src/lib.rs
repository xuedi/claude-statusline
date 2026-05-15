pub mod api;
pub mod bar;
pub mod cache;
pub mod error;
pub mod input;
pub mod segments;
pub mod time;

use input::Input;

pub fn render(raw: &str) -> String {
    if raw.trim().is_empty() {
        return "Claude".to_string();
    }
    let data: Input = serde_json::from_str(raw).unwrap_or_default();
    segments::all(&data).join(" | ")
}

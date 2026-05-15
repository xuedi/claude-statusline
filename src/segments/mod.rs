use crate::input::Input;

pub mod effort;
pub mod git;
pub mod model;
pub mod rate_limits;
pub mod tokens;

pub fn all(input: &Input) -> Vec<String> {
    let mut parts = Vec::with_capacity(8);
    parts.push(model::render(input));
    if let Some(seg) = git::render(input) {
        parts.push(seg);
    }
    if let Some(seg) = tokens::render(input) {
        parts.push(seg);
    }
    parts.push(format!("effort: {}", effort::render()));
    parts.extend(rate_limits::render(input));
    parts
}

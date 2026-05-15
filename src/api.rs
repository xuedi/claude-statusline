use std::env;
use std::fs;
use std::time::Duration;

use serde_json::Value;

use crate::cache::CACHE_DIR;
use crate::error::{Error, Result};

const USAGE_ENDPOINT: &str = "https://api.anthropic.com/api/oauth/usage";
const USER_AGENT: &str = "claude-code/2.1.34";
const OAUTH_BETA: &str = "oauth-2025-04-20";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

pub fn fetch_usage(cache_file: &str) -> Result<Value> {
    let home = env::var("HOME").unwrap_or_default();
    let creds_path = format!("{home}/.claude/.credentials.json");

    let creds_raw = fs::read_to_string(&creds_path).map_err(|_| Error::MissingCredentials)?;
    let creds: Value = serde_json::from_str(&creds_raw)?;
    let token = creds["claudeAiOauth"]["accessToken"]
        .as_str()
        .ok_or(Error::MissingField("claudeAiOauth.accessToken"))?
        .to_string();

    let _ = fs::create_dir_all(CACHE_DIR);
    // WHY: write an empty file before the request to act as a stampede lock -
    // a parallel statusline invocation finds the file fresh and skips its own fetch.
    let _ = fs::write(cache_file, "");

    let agent = ureq::AgentBuilder::new().timeout(REQUEST_TIMEOUT).build();

    let body = agent
        .get(USAGE_ENDPOINT)
        .set("Accept", "application/json")
        .set("Authorization", &format!("Bearer {token}"))
        .set("anthropic-beta", OAUTH_BETA)
        .set("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| Error::Http(e.to_string()))?
        .into_string()
        .map_err(|e| Error::Http(e.to_string()))?;

    let data: Value = serde_json::from_str(&body)?;
    if data.get("five_hour").is_none() {
        return Err(Error::MissingField("five_hour"));
    }
    let _ = fs::write(cache_file, &body);
    Ok(data)
}

use askama::Result;
use once_cell::sync::Lazy;
use regex::Regex;

static RE: Lazy<Regex> = Lazy::new(|| regex::Regex::new(r"\b([^ .,]) ").unwrap());
pub fn htmlize(s: &str) -> Result<String> {
    Ok(RE.replace_all(s, "$1&nbsp;").replace('\n', "<br>"))
}

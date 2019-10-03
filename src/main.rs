
use reqwest::get;
use serde_json::{from_reader, Value};

#[derive(Debug)]
struct PullRequest {
    url: String,
    title: String,
    body: String,
    target_ref: String,
    user_login: String,
    base_ref: String,
}

fn lookup(value: &Value, key: &str, second_key: Option<&str>) -> Option<String> {
    match second_key {
        None => match value.get(key)? {
            Value::String(s) => Some(s.clone()),
            _ => None
        },
        Some(key2) => match value.get(key)? {
            Value::Object(m) => match m.get(key2)? {
                Value::String(s) => Some(s.clone()),
                _ => None
            },
            _ => None
        }
    }
}

impl PullRequest {
    pub fn new(value: Value) -> Self {
        Self {
            url: lookup(&value, "url", None).expect("Could not access url from response!"),
            title: lookup(&value, "title", None).expect("Could not access title from response!"),
            body: lookup(&value, "body", None).expect("Could not access body from response!"),
            target_ref: lookup(&value, "base", Some("ref")).expect("Could not access target_ref from response!"),
            user_login: lookup(&value, "user", Some("login")).expect("Could not access user_login from response!"),
            base_ref: lookup(&value, "head", Some("ref")).expect("Could not access base_ref from response!"),
        }
    }
}

fn main() {
    let stream =
        get("https://api.github.com/repos/apache/arrow/pulls/5542")
        .expect("Could not get PR info from GitHub.");

    let json: Value =
        from_reader(stream)
            .expect("Could not convert to JSON.");

    let pr = PullRequest::new(json);

    dbg!(pr);
}

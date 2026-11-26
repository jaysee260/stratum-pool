use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: Value,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Debug)]
pub struct Subscribe {
    user_agent: Option<String>,
}

#[derive(Debug)]
pub struct Authorize {
    worker_name: String,
    password: Option<String>,
}

#[derive(Debug)]
pub struct Submit {
    worker_name: String,
    job_id: String,
    extranonce2: String,
    ntime: String,
    nonce: String,
}

#[derive(Debug)]
pub enum Message {
    Subscribe(Subscribe),
    Authorize(Authorize),
    Submit(Submit),
    SuggestDifficulty(SuggestDifficulty),
    Configure(Configure),
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Value,
    pub result: Value,
    pub error: Value,
}

#[derive(Debug)]
pub struct SuggestDifficulty {
    pub difficulty: f64,
}

#[derive(Debug)]
pub struct Configure {
    pub version_rolling_mask: Option<u32>,
}

fn parse_subscribe(request: &Request) -> Result<Subscribe, String> {
    let user_agent = extract_optional_field(&request.params, 0, "user_agent")?;
    Ok(Subscribe { user_agent })
}

fn parse_authorize(request: &Request) -> Result<Authorize, String> {
    let worker_name = extract_required_field(&request.params, 0, "worker_name")?;
    let password = extract_optional_field(&request.params, 1, "password")?;
    Ok(Authorize {
        worker_name,
        password,
    })
}

fn parse_submit(request: &Request) -> Result<Submit, String> {
    let worker_name = extract_required_field(&request.params, 0, "worker_name")?;
    let job_id = extract_required_field(&request.params, 1, "job_id")?;
    let extranonce2 = extract_required_field(&request.params, 2, "extranonce2")?;
    let ntime = extract_required_field(&request.params, 3, "ntime")?;
    let nonce = extract_required_field(&request.params, 4, "nonce")?;

    Ok(Submit {
        worker_name,
        job_id,
        extranonce2,
        ntime,
        nonce,
    })
}

pub fn parse_message(request: &Request) -> Result<Message, String> {
    match request.method.as_str() {
        "mining.subscribe" => Ok(Message::Subscribe(parse_subscribe(request)?)),
        "mining.authorize" => Ok(Message::Authorize(parse_authorize(request)?)),
        "mining.submit" => Ok(Message::Submit(parse_submit(request)?)),
        "mining.suggest_difficulty" => Ok(Message::SuggestDifficulty(parse_suggest_difficulty(request)?)),
        "mining.configure" => Ok(Message::Configure(parse_configure(request)?)),
        _ => Err(format!("unsupported method: {}", request.method)),
    }
}

fn extract_required_field(params: &[Value], index: usize, field: &str) -> Result<String, String> {
    match params.get(index) {
        None => Err(format!("{field} cannot be empty")),
        Some(value) => match value.as_str() {
            None => Err(format!("{field} must be a string")),
            Some(text) => Ok(text.to_string()),
        },
    }
}

fn extract_optional_field(
    params: &[Value],
    index: usize,
    field: &str,
) -> Result<Option<String>, String> {
    match params.get(index) {
        None => Ok(None),
        Some(value) => match value.as_str() {
            // is it a string?
            None => Err(format!("{field} must be a string")),
            Some(text) => Ok(Some(text.to_string())),
        },
    }
}

fn parse_suggest_difficulty(request: &Request) -> Result<SuggestDifficulty, String> {
    match request.params.get(0) {
        None => Err("difficulty cannot be empty".to_string()),
        Some(value) => match value.as_f64() { // is it a number?
            None => Err("difficulty must be a number".to_string()),
            Some(difficulty) => Ok(SuggestDifficulty { difficulty }),
        }
    }
}

fn parse_configure(request: &Request) -> Result<Configure, String> {
    // 1. the settings object is the second param
    let mask = match request.params.get(1) {
        None => None,
        Some(settings) => settings.get("version-rolling.mask"),
    };

    // 2. no mask means the miner didn't ask for version rolling
    let value = match mask {
        None => return Ok(Configure { version_rolling_mask: None }),
        Some(value) => value,
    };

    // 3. the mask must be hex text; turn it into a number
    match value.as_str() {
        None => Err("version-rolling.mask must be a string".to_string()),
        Some(text) => match u32::from_str_radix(text, 16) {
            Err(_) => Err("version-rolling.mask must be hex".to_string()), // is it in hex format?
            Ok(mask) => Ok(Configure { version_rolling_mask: Some(mask) }),
        }
    }
}

#[cfg(test)]
mod tests;

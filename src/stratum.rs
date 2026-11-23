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
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Value,
    pub result: Value,
    pub error: Value
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

#[cfg(test)]
mod tests;

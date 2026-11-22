use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Request {
    id: Value,
    method: String,
    params: Vec<Value>
}

#[derive(Debug)]
struct Subscribe {
    user_agent: Option<String>
}

#[derive(Debug)]
struct Authorize {
    worker_name: String,
    password: Option<String>
}

fn parse_subscribe(request: &Request) -> Result<Subscribe, String> {
    let user_agent = extract_optional_field(&request.params, 0, "user_agent")?;
    Ok(Subscribe { user_agent })
}

fn parse_authorize(request: &Request) -> Result<Authorize, String> {
    let worker_name = extract_required_field(&request.params, 0, "worker_name")?;
    let password = extract_optional_field(&request.params, 1, "password")?;
    Ok(Authorize { worker_name, password })
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

fn extract_optional_field(params: &[Value], index: usize, field: &str) -> Result<Option<String>, String> {
    match params.get(index) {
        None => Ok(None),
        Some(value) => match value.as_str() { // is it a string?
            None => Err(format!("{field} must be a string")),
            Some(text) => Ok(Some(text.to_string())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_subscribe_request() {
        let raw_message = r#"{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
        let request: Request = serde_json::from_str(raw_message).unwrap();
        assert_eq!(request.id, 1);
        assert_eq!(request.method, "mining.subscribe");
        assert_eq!(request.params, ["bitaxe/BM1366/v2.4.0"]);
    }

    #[test]
    fn parses_string_id() {
        let raw_message = r#"{"id": "1", "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
        let request: Request = serde_json::from_str(raw_message).unwrap();
        assert_eq!(request.id, "1");
    }

    #[test]
    fn parses_null_id() {
        // if an id is null, the message is a notification and no response is needed
        let raw_message = r#"{"id": null, "method": "mining.set_difficulty", "params": [512]}"#;
        let request: Request = serde_json::from_str(raw_message).unwrap();
        assert!(request.id.is_null());
    }

    #[test]
    fn rejects_missing_params() {
        let raw_message = r#"{"id": 1, "method": "mining.subscribe"}"#;
        let result: Result<Request, _> = serde_json::from_str(raw_message);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_json_line() {
        let raw_message = "GET / HTTP/1.1";
        let result: Result<Request, _> = serde_json::from_str(raw_message);
        assert!(result.is_err());
    }

    #[test]
    fn parses_request_without_params() {
        let raw_message = r#"{"id": 1, "method": "mining.subscribe", "params": []}"#;
        let request: Request = serde_json::from_str(raw_message).unwrap();
        let subscribe = parse_subscribe(&request).unwrap();
        assert_eq!(subscribe.user_agent, None);
    }

        #[test]
    fn parses_request_with_params() {
        let raw_message = r#"{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
        let request: Request = serde_json::from_str(raw_message).unwrap();
        let subscribe = parse_subscribe(&request).unwrap();
        assert_eq!(subscribe.user_agent, Some("bitaxe/BM1366/v2.4.0".to_string()));
    }

    #[test]
    fn rejects_non_text_params() {
        let raw_message = r#"{"id": 1, "method": "mining.subscribe", "params": [42]}"#;
        let request: Request = serde_json::from_str(raw_message).unwrap();
        let result = parse_subscribe(&request);
        assert!(result.is_err());
    }

    #[test]
    fn parses_authorize() {
        let input = r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1", "x"]}"#;
        let request: Request = serde_json::from_str(input).unwrap();
        let authorize = parse_authorize(&request).unwrap();
        assert_eq!(authorize.worker_name, "bc1qexampleaddress.bitaxe1");
        assert_eq!(authorize.password, Some("x".to_string()));
    }

    #[test]
    fn parses_authorize_without_password() {
        let input = r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1"]}"#;
        let request: Request = serde_json::from_str(input).unwrap();
        let authorize = parse_authorize(&request).unwrap();
        assert_eq!(authorize.worker_name, "bc1qexampleaddress.bitaxe1");
        assert!(authorize.password.is_none());
    }

    #[test]
    fn rejects_authorize_without_worker_name() {
        let input = r#"{"id": 2, "method": "mining.authorize", "params": []}"#;
        let request: Request = serde_json::from_str(input).unwrap();
        let result: Result<Authorize, String> = parse_authorize(&request);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_authorize_with_non_string_worker_name() {
        let input = r#"{"id": 2, "method": "mining.authorize", "params": [42, "x"]}"#;
        let request: Request = serde_json::from_str(input).unwrap();
        let result: Result<Authorize, _> = parse_authorize(&request);
        assert!(result.is_err())
    }

    #[test]
    fn rejects_authorize_with_non_string_password() {
        let input = r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1", 36]}"#;
        let request: Request = serde_json::from_str(input).unwrap();
        let result: Result<Authorize, _> = parse_authorize(&request);
        assert!(result.is_err())
    }
}
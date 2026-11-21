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

fn parse_subscribe(request: &Request) -> Result<Subscribe, String> {
    match request.params.first() {
        None => Ok(Subscribe { user_agent: None }),
        Some(value) => match value.as_str() {
            None => Err("user_agent must be a string".to_string()),
            Some(name) => Ok(Subscribe { user_agent: Some(name.to_string()) } ),
        }
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
}
use super::*;

#[test]
fn parses_subscribe_request() {
    let input = r#"{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert_eq!(request.id, 1);
    assert_eq!(request.method, "mining.subscribe");
    assert_eq!(request.params, ["bitaxe/BM1366/v2.4.0"]);
}

#[test]
fn parses_string_id() {
    let input = r#"{"id": "1", "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert_eq!(request.id, "1");
}

#[test]
fn parses_null_id() {
    // if an id is null, the message is a notification and no response is needed
    let input = r#"{"id": null, "method": "mining.set_difficulty", "params": [512]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert!(request.id.is_null());
}

#[test]
fn rejects_missing_params() {
    let input = r#"{"id": 1, "method": "mining.subscribe"}"#;
    let result: Result<Request, _> = serde_json::from_str(input);
    assert!(result.is_err());
}

#[test]
fn rejects_non_json_line() {
    let input = "GET / HTTP/1.1";
    let result: Result<Request, _> = serde_json::from_str(input);
    assert!(result.is_err());
}

#[test]
fn parses_request_without_params() {
    let input = r#"{"id": 1, "method": "mining.subscribe", "params": []}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let subscribe = parse_subscribe(&request).unwrap();
    assert_eq!(subscribe.user_agent, None);
}

#[test]
fn parses_request_with_params() {
    let input = r#"{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let subscribe = parse_subscribe(&request).unwrap();
    assert_eq!(
        subscribe.user_agent,
        Some("bitaxe/BM1366/v2.4.0".to_string())
    );
}

#[test]
fn rejects_non_text_params() {
    let input = r#"{"id": 1, "method": "mining.subscribe", "params": [42]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let result = parse_subscribe(&request);
    assert!(result.is_err());
}

#[test]
fn parses_authorize() {
    let input =
        r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1", "x"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let authorize = parse_authorize(&request).unwrap();
    assert_eq!(authorize.worker_name, "bc1qexampleaddress.bitaxe1");
    assert_eq!(authorize.password, Some("x".to_string()));
}

#[test]
fn parses_authorize_without_password() {
    let input =
        r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1"]}"#;
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
    let input =
        r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1", 36]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert!(parse_authorize(&request).is_err())
}

#[test]
fn parses_submit() {
    let input = r#"{"id": 4, "method": "mining.submit", "params": ["bc1qexampleaddress.bitaxe1", "1a2b", "0000000000000001", "674a1b2c", "a1b2c3d4"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let submit = parse_submit(&request).unwrap();
    assert_eq!(submit.worker_name, "bc1qexampleaddress.bitaxe1");
    assert_eq!(submit.job_id, "1a2b");
    assert_eq!(submit.extranonce2, "0000000000000001");
    assert_eq!(submit.ntime, "674a1b2c");
    assert_eq!(submit.nonce, "a1b2c3d4")
}

#[test]
fn rejects_submit_without_nonce() {
    let input = r#"{"id": 4, "method": "mining.submit", "params": ["bc1qexampleaddress.bitaxe1", "1a2b", "0000000000000001", "674a1b2c"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert!(parse_submit(&request).is_err())
}

#[test]
fn rejects_non_string_submit_fields() {
    let input = r#"{"id": 4, "method": "mining.submit", "params": ["bc1qexampleaddress.bitaxe1", 46, "0000000000000001", "674a1b2c", "a1b2c3d4"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert!(parse_submit(&request).is_err())
}

#[test]
fn parses_message_subscribe() {
    let input = r#"{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let message = parse_message(&request).unwrap();
    assert!(matches!(message, Message::Subscribe(_)));
}

#[test]
fn parses_message_authorize() {
    let input =
        r#"{"id": 2, "method": "mining.authorize", "params": ["bc1qexampleaddress.bitaxe1", "x"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let message = parse_message(&request).unwrap();
    assert!(matches!(message, Message::Authorize(_)))
}

#[test]
fn parses_message_submit() {
    let input = r#"{"id": 4, "method": "mining.submit", "params": ["bc1qexampleaddress.bitaxe1", "1a2b", "0000000000000001", "674a1b2c", "a1b2c3d4"]}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    let message = parse_message(&request).unwrap();
    assert!(matches!(message, Message::Submit(_)));
}

#[test]
fn rejects_invalid_message_type() {
    let input = r#"{ "id": 1, "method": "invalid.method", "params": []}"#;
    let request: Request = serde_json::from_str(input).unwrap();
    assert!(parse_message(&request).is_err())
}

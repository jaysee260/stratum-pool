use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use serde_json::json;

use crate::stratum::{parse_message, Message};

mod stratum;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    println!("listening on 127.0.0.1:3333");

    loop {
        let (stream, address) = listener.accept()?;
        println!("connection from {address}");

        let mut writer = match stream.try_clone() {
            Ok(writer) => writer,
            Err(error) => {
                println!("clone failed: {error}");
                continue;
            }
        };
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    println!("read failed: {error}");
                    break;
                }
            };
            let request: stratum::Request = match serde_json::from_str(&line) {
                Ok(request) => request,
                Err(error) => {
                    println!("not a request: {error}");
                    continue;
                }
            };

            let message = match parse_message(&request) {
                Ok(message) => message,
                Err(error) => {
                    println!("not a message: {error}");
                    continue;
                }
            };

            println!("received message: {message:?}");
            let is_authorize = matches!(message, Message::Authorize(_));
            let result = match message {
                Message::Subscribe(_) => json!([
                    [["mining.set_difficulty", "1"], ["mining.notify", "1"]],
                    "00000001",
                    8
                ]),
                Message::Authorize(_) => json!(true),
                Message::Submit(_) => json!(true),
            };

            let response = stratum::Response {
                id: request.id.clone(),
                result,
                error: json!(null),
            };

            let text = serde_json::to_string(&response).unwrap();
            if let Err(error) = writeln!(writer, "{text}") {
                println!("write failed: {error}");
                break;
            };

            if is_authorize {
                let notification = stratum::Request {
                    id: json!(null),
                    method: "mining.set_difficulty".to_string(),
                    params: vec![json!(1)],
                };
                let text = serde_json::to_string(&notification).unwrap();
                if let Err(error) = writeln!(writer, "{text}") {
                    println!("write failed: {error}");
                    break;
                }
            }
        }

        println!("connection closed");
    }
}

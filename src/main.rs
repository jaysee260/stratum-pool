use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

use serde_json::json;

use crate::stratum::parse_message;

mod stratum;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    println!("listening on 127.0.0.1:3333");

    let (stream, address) = listener.accept()?;
    println!("connection from {address}");

    let mut writer = stream.try_clone()?;
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line?; // shadowing?
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
        let response = stratum::Response {
            id: request.id.clone(),
            result: json!(true),
            error: json!(null),
        };

        let text = serde_json::to_string(&response).unwrap();
        writeln!(writer, "{text}")?;
    }

    println!("connection closed");

    Ok(())
}

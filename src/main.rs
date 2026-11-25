use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpListener, TcpStream};

use serde_json::json;

use crate::stratum::{parse_message, Message};

mod stratum;

const MAX_LINE_BYTES: u64 = 4096;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333").await?;
    println!("listening on 127.0.0.1:3333");
    let mut next_id: u32 = 1;

    loop {
        let (stream, address) = listener.accept().await?;
        println!("connection from {address}");
        tokio::spawn(handle_connection(stream, next_id));
        next_id += 1;
    }
}

async fn handle_connection(stream: TcpStream, extranonce1: u32) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    while let Some(line) = read_line(&mut reader).await {
        // todo: parse_line
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
                format!("{extranonce1:08x}"),
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
        if let Err(error) = writer.write_all(format!("{text}\n").as_bytes()).await {
            println!("write failed: {error}");
            break;
        }

        if is_authorize {
            let notification = stratum::Request {
                id: json!(null),
                method: "mining.set_difficulty".to_string(),
                params: vec![json!(1)],
            };
            let text = serde_json::to_string(&notification).unwrap();
            if let Err(error) = writer.write_all(format!("{text}\n").as_bytes()).await {
                println!("write failed: {error}");
                break;
            }
        }
    }

    println!("connection closed");
}

async fn read_line(reader: &mut BufReader<OwnedReadHalf>) -> Option<String> {
    let mut line = String::new();
    let mut limited = reader.take(MAX_LINE_BYTES);

    match limited.read_line(&mut line).await {
        Ok(0) => None,
        Ok(_) if !line.ends_with('\n') => {
            println!("line too long or incomplete, closing");
            None
        }
        Ok(_) => Some(line),
        Err(error) => {
            println!("read failed: {error}");
            None
        }
    }
}

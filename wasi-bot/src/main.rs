use bytecodec::DecodeExt;
use http_req::request;

use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::{io::{Read, Write}, println};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

const BOT_URL: &str= "https://dipankardas011-gpt2-bot.hf.space/generate";
const BOT_TEXT_FIELD: &str = "text";

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    let request_path = req.request_target().to_string();
    let request_type = req.method().to_string();

    return match (request_type.as_str(), request_path.as_str()) {
        ("GET", "/ping") => {

            println!("request target [{request_path}] method [{request_type}]");
            Ok(Response::new(
                HttpVersion::V1_1,
                StatusCode::new(200)?,
                ReasonPhrase::new("")?,
                format!("Pong!!")))
        }
        ("POST", "/bot") => {

            let mod_req: String = req.body().replace(" ", "%20");

            if mod_req.len() as i32 == 0 {
                Ok(Response::new(
                    HttpVersion::V1_1,
                    StatusCode::new(401)?,
                    ReasonPhrase::new("")?,
                    format!("Invalid request no promppt found!")))
            }else {

                let bot_uri = format!("{BOT_URL}?{BOT_TEXT_FIELD}={mod_req}");

                let mut writer = Vec::new(); //container for body of a response
                let res = request::get(bot_uri, &mut writer).unwrap();
                let bot_response = String::from_utf8_lossy(&writer);
                println!("Status: {} {}", res.status_code(), res.reason());
                println!("Headers {}", res.headers());
                if res.status_code() == http_req::response::StatusCode::new(200) {
                    Ok(Response::new(
                        HttpVersion::V1_1,
                        StatusCode::new(200)?,
                        ReasonPhrase::new("")?,
                        format!("Generated Response {bot_response}")))
                } else {
                    Ok(Response::new(
                        HttpVersion::V1_1,
                        StatusCode::new(503)?,
                        ReasonPhrase::new("")?,
                        format!("Server side error")))
                }
            }
        }
        _ => {
            println!("request target [{request_path}] method [{request_type}]");
            Ok(Response::new(
                HttpVersion::V1_1,
                StatusCode::new(403)?,
                ReasonPhrase::new("")?,
                format!("Route path or method is invalid!")))
        }
    };
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}

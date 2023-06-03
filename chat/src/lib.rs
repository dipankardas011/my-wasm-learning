use std::println;

use anyhow::Result;
use reqwest;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

// A simple Spin HTTP component.
#[http_component]
#[tokio::main]
async fn handle_chat(req: Request) -> Result<Response> {
    let body = req.body().iter().enumerate();
    let mut mod_req: String = String::new();
    for (_, item) in body {
        let b = std::str::from_utf8(item).unwrap();
        mod_req = b.replace(" ", "%20");
    }

    println!("{mod_req}");

    let bot_uri = format!("https://dipankardas011-gpt2-bot.hf.space/generate?text={mod_req}");

    if mod_req.len() as i32 > 0 {

        println!("{bot_uri}");

        match foo(bot_uri).await {
            Ok(bot_response) => {
                println!("Request completed {}", bot_response);
            },
            Err(e) => println!("Error occurred: {:?}", e),
        }

        Ok(http::Response::builder()
            .status(200)
            .header("foo", "bar")
            .body(Some(mod_req.into()))?)
    } else {

        Ok(http::Response::builder()
            .status(403)
            .header("request", "invalid")
            .body(Some("add -d flag for the text to pass".into()))?)
    }
}

async fn foo(bot_url: String) -> Result<String, reqwest::Error> {
    let response = reqwest::get(bot_url).await?;

    let mut resp = String::new();

    if response.status() == reqwest::StatusCode::OK {
        let text = response.text().await?;
        resp = text;
    } else {
        println!("Response was not 200 OK");
    }

    Ok(resp)
}

// #[tokio::main]
// async fn main() {
//     match foo().await {
//         Ok(_) => println!("Request completed"),
//         Err(e) => println!("Error occurred: {:?}", e),
//     }
// }

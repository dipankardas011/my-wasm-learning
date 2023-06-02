use std::println;

// use anyhow::Result;
// use reqwest;
// use spin_sdk::{
//     http::{Request, Response},
//     // http_component,
// };

/// A simple Spin HTTP component.
// #[http_component]
// fn handle_chat(req: Request) -> Result<Response> {
//     let body = req.body().iter().enumerate();
//     let mut mod_req: String = String::new();
//     for (_, item) in body {
//         let b = std::str::from_utf8(item).unwrap();
//         mod_req = b.replace(" ", "%20");
//     }

//     println!("{mod_req}");

//     let bot_uri = format!("https://dipankardas011-gpt2-bot.hf.space/generate?text={mod_req}");

//     if mod_req.len() as i32 > 0 {

//         println!("{bot_uri}");

//         foo();

//         Ok(http::Response::builder()
//             .status(200)
//             .header("foo", "bar")
//             .body(Some(mod_req.into()))?)
//     } else {

//         Ok(http::Response::builder()
//             .status(403)
//             .header("request", "invalid")
//             .body(Some("add -d flag for the text to pass".into()))?)
//     }
//     // Ok(http::Response::builder()
//     //     .status(200)
//     //     .header("foo", "bar")
//     //     .body(Some("Hello, Fermyon".into()))?)
// }
// use futures::executor::block_on;
// fn main() {
//     block_on(foo());
// }

// async fn foo() {
//     match reqwest::get("http://youtube.local/hello").await {
//         Ok(mut response) => {
//             if response.status() == reqwest::StatusCode::OK {
//                 match response.text().await {
//                     Ok(text) => println!("Response Text: {}", text),
//                     Err(_) => println!("Could not read response text!"),
//                 }
//             } else {
//                 println!("Response was not 200 OK")
//             }
//         }
//         Err(_) => println!("Could not make the request")
//     }
// }
// Add the required dependencies to Cargo.toml
// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }
// tokio = { version = "1", features = ["full"] }

use reqwest;

#[tokio::main]
async fn main() {
    match foo().await {
        Ok(_) => println!("Request completed"),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}

async fn foo() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.google.com").await?;

    if response.status() == reqwest::StatusCode::OK {
        let text = response.text().await?;
        println!("Response Text: {}", text);
    } else {
        println!("Response was not 200 OK");
    }

    Ok(())
}

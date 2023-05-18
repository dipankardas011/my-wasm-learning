use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_ksctl(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let abcd: &str = r#"
    <html>
    <body>
        <h1> Welcome to kubesimplify ksctl WASM!</h1>
        <p> here are the various HTTP routes </p>
        <ul>
        <li> /create </li>
        <li> /delete </li>
        </ul>
    </body>
    </html>
    "#;
    Ok(http::Response::builder()
        .status(200)
        .header("about", "ksctl")
        .body(Some(abcd.into()))?)
}

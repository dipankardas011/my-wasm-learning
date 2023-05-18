use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_delete_cluster(req: Request) -> Result<Response> {
    let body: &str = r#"
    <html>
        <body>
        <h1>ksctl delete-cluster local</h1>
        deleted!
        </body>
    </html>
    "#;
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("delete-cluster", "local")
        .body(Some(body.into()))?)
}

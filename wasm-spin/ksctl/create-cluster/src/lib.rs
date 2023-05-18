use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::str;


/// A simple Spin HTTP component.
#[http_component]
fn handle_create_cluster(req: Request) -> Result<Response> {
    println!("{:?}", req.method());


    let str = req.body().iter().enumerate();

    let ip = String::from("127.0.0.1");
    let mut clustername: &str = "N/A";

    for (_, item) in str {
        let b = std::str::from_utf8(item).unwrap();
        let params: std::str::Split<&str> = b.split(",");

        let mut got_it: bool = true;
        for info in params{
            if got_it {
                clustername = info;
                got_it = false;
            }
        }
    }


    let kubeconfig = format!("
    apiVersion: v1\n
    kind: Config\n
    clusters:\n
    - cluster:\n
        server: https://{}:6443\n
      name: {}\n
    users:\n
    - name: admin\n
    contexts:\n
    - context:\n
      name: default", ip, clustername);



    let body = format!(r#"
    <html>
        <body>
        <h1>ksctl create-cluster local</h1>
        <pre> {} </pre>
        </body>
    </html>
    "#, kubeconfig);


    Ok(http::Response::builder()
        .status(200)
        .header("create-cluster", "local")
        .body(Some(body.into()))?)
}


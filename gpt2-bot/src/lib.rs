use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};


const BOT_URL: &str= "https://dipankardas011-gpt2-bot.hf.space/generate";
const BOT_TEXT_FIELD: &str = "text";


fn render_template(data: &str) -> String {
    format!(
        "
<!DOCTYPE html>
<html data-bs-theme=\"dark\">
    <head>
        <meta charset=\"utf-8\">
        <title>Rust based GPT-2 bot</title>
        <link rel=\"stylesheet\" href=\"https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css\" integrity=\"sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T\" crossorigin=\"anonymous\">
    </head>
    <body class=\"bg-dark\">
        <div class=\"container\">
            <h1 class=\"text-center mt-5 text-white\">Rust based GPT-2 bot</h1>
            <form action=\"/bot\" method=\"post\" class=\"d-flex flex-column align-items-center\">
                <label for=\"query\" class=\"mb-2 text-white\">Enter your text:</label>
                <input type=\"text\" name=\"query\" id=\"query\" class=\"form-control w-100 p-2 mb-4 border border-gray-300 rounded\">
                <button type=\"submit\" class=\"btn btn-primary w-100 py-2 px-4 text-white rounded\">Get Answer</button>
            </form>
            <h2 class=\"text-white\">Answer:</h2>
            <p class=\"text-white\">{}</p>
        </div>
    </body>
</html>
        ",
        data
    )
}


/// A simple Spin HTTP component.
#[http_component]
fn handle_wasi_bot(req: Request) -> Result<Response> {
    let mut request_path = req.uri().to_string();
    let request_method = req.method();

    let request_body = match req.body() {
        Some(body) => match std::str::from_utf8(body) {
            Ok(v) => v.to_owned().replace(" ", "%20"), // v is now String
            Err(e) => format!("Invalid UTF-8 sequence(Request): {}", e),
        },
        None => {
            request_path = String::from("nil");
            format!("No request body")
        },
    };

    println!("{request_method} {request_path} {request_body}");

    match (request_method.as_str(), request_path.as_str()) {
        ("GET", "/") => {
            Ok(http::Response::builder()
            .status(200)
            .body(Some(render_template("Backend HELATHY!").into()))?)
        },
        ("POST", "/bot") => {
            let bot_uri = format!("{BOT_URL}?{BOT_TEXT_FIELD}={request_body}");
            
            let res = spin_sdk::outbound_http::send_request(
                http::Request::builder()
                    .uri(bot_uri)
                    .body(None)?,
            )?;

            let res_body = match res.body() {
                Some(body) => match std::str::from_utf8(body) {
                    Ok(v) => v.to_owned(), // v is now String
                    Err(e) => format!("Invalid UTF-8 sequence(Response): {}", e),
                },
                None => format!("No response body"),
            };
                
            Ok(http::Response::builder()
                .status(200)
                .body(Some(render_template(res_body.as_str()).into()))?)
        },
        _ => {
            Ok(http::Response::builder()
            .status(404)
            .header("foo", "bar")
            .body(Some("Invalid request".into()))?)
        }
    }
    
}

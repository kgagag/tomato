use tiny_http::{Server, Response, Request};

fn handle_request(request: Request) {
    match request.url() {
        "/" => {
            let html = r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Home Page</title>
                </head>
                <body>
                    <h1>Welcome to the Home Page</h1>
                    <p>This is a simple HTTP server in Rust using tiny_http.</p>
                </body>
                </html>
            "#;

            let response = Response::from_string(html)
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
            request.respond(response).unwrap();
        }
        _ => {
            let not_found = "404 Not Found";
            let response = Response::from_string(not_found)
                .with_status_code(404);
            request.respond(response).unwrap();
        }
    }
}

#[test]
pub fn run_http_server() {
    let server = Server::http("0.0.0.0:3000").unwrap();
    println!("Listening on http://0.0.0.0:3000");

   
}



use serde_json;
use tide::{Response, StatusCode};

///
fn generate_response(status_code: StatusCode) -> Response {
    Response::builder(StatusCode::Unauthorized)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({
            "success": status_code.is_success(),
            "status_code": status_code.to_string()
        }))
        .build()
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("[ Status code Demo ]\n");

    tide::log::start();

    let mut server = tide::new();

    //
    server
        .at("/status-code-test-1")
        .get(|_req| async move { Ok(generate_response(StatusCode::Unauthorized)) });

    server
        .at("/status-code-test-2")
        .get(|_req| async move { Ok(generate_response(StatusCode::BadRequest)) });

    server
        .at("/status-code-test-3")
        .get(|_req| async move { Ok(generate_response(StatusCode::NotFound)) });

    server
        .at("/status-code-test-4")
        .get(|_req| async move { Ok(generate_response(StatusCode::InternalServerError)) });

    // Start listening on speficied address and port
    let listen_to = "0.0.0.0:8080";
    println!("Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;

    println!("Server is closed.");
    Ok(())
}

use serde_json;
use tide::{Middleware, Next, Request, Response, Result, StatusCode};

/// Token-based authentication middleware
struct TokenBasedAuthenticationMiddleware {}

///
#[async_trait::async_trait]
impl<State> Middleware<State> for TokenBasedAuthenticationMiddleware
where
    State: Clone + Send + Sync + 'static,
{
    ///
    async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> Result {
        // Get the token value from `Authentication` header
        let mut token_str = "";
        if let Some(authorization) = request.header("Authorization") {
            token_str = authorization.as_str();
        }

        // Replace to your complicated token verification logic here:
        let is_valid_token = token_str == "demo-token";

        // If token is invalid, then rejct immediately
        if !is_valid_token {
            let reject_response = Response::builder(StatusCode::Unauthorized)
                .header("Content-Type", "applicationx/json")
                .body(serde_json::json!({
                    "errorCode": StatusCode::Unauthorized.to_string(),
                    "errorMessage": "Token is invalid."
                }))
                .build();

            return Ok(reject_response);
        }

        // Go forward to the next middleware or request handler
        Ok(next.run(request).await)
    }
}

///
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("[ Middleware Demo ]\n");

    tide::log::start();

    let mut server = tide::new();

    // Apply the middleware
    server.with(TokenBasedAuthenticationMiddleware {});

    // Authenticated route
    server.at("/home").get(|_req| async move {
        let res = Response::builder(StatusCode::Ok)
            .header("Content-Type", "applicationx/json")
            .body(serde_json::json!({
                "user": {
                    "name": "Wison",
                    "role": "Administrator"
                }
            }))
            .build();
        Ok(res)
    });

    // Start listening on speficied address and port
    let listen_to = "0.0.0.0:8080";
    println!("Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;

    println!("Server is closed.");
    Ok(())
}

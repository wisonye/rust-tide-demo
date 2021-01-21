use serde_json;
use tide::prelude::{Deserialize, Serialize};
use tide::{Middleware, Next, Request, Response, Result, StatusCode};

const JWT_SECRET_KEY: &'static [u8] = b"My Super Secret Key!";

///
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    role: String,
}

///
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

///
mod jwt_util {
    use crate::User;
    use crate::JWT_SECRET_KEY;
    use hmac::{Hmac, NewMac};
    use jwt::{SignWithKey, VerifyWithKey};
    use sha2::Sha256;
    use std::collections::BTreeMap;

    /// `jwt.sign_with_key()` support types below:
    ///
    /// - `BTreeMap::<&str, serde_json::value::Value>`
    /// - `BTreeMap::<&str, String>` // We use this type!
    /// - `BTreeMap::<&str, &str>`
    ///
    pub fn generate_token(user: User) -> String {
        let jwt_key: Hmac<Sha256> = Hmac::new_varkey(&JWT_SECRET_KEY).unwrap();
        let mut map_to_sign = BTreeMap::<&str, String>::new();
        // map_to_sign.insert("user", serde_json::to_value(user).unwrap());
        map_to_sign.insert("user", serde_json::to_string(&user).unwrap());

        let token_str = map_to_sign.sign_with_key(&jwt_key).unwrap();
        token_str
    }

    ///
    pub fn verfiy_token(token: &str) -> Option<serde_json::value::Value> {
        let jwt_key: Hmac<Sha256> = Hmac::new_varkey(&JWT_SECRET_KEY).unwrap();

        match token.verify_with_key(&jwt_key) {
            Ok(decoded_value) => {
                println!(
                    "[ JwtUtil --> verfiy_token, decoded_value: {:#?}",
                    &decoded_value
                );
                Some(decoded_value)
            }
            Err(error) => {
                println!("[ JwtUtil --> verfiy_token, error {:#?}", &error);
                None
            }
        }
    }
}

/// JWT authentication middleware
struct JwtAuthenticationMiddleware {}

///
#[async_trait::async_trait]
impl<State> Middleware<State> for JwtAuthenticationMiddleware
where
    State: Clone + Send + Sync + 'static,
{
    ///
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
        // Skip the token checking for `/auth` route!!!
        println!(
            "[ Middleware trait --> handle ] - url path: {}",
            request.url().path()
        );
        if request.url().path() == "/auth" {
            // Go forward to the next middleware or request handler
            return Ok(next.run(request).await);
        }

        // Get the token value from `Authentication` header
        let mut token_str = "";
        if let Some(authorization) = request.header("Authorization") {
            token_str = authorization.as_str();
        }
        println!("[ Middleware trai --> handle ] - token_str: {}", &token_str);

        // Replace to your complicated token verification logic here:
        // Replace to your complicated token verification logic here:
        // Replace to your complicated token verification logic here:
        match jwt_util::verfiy_token(token_str) {
            Some(decoded_token) => {
                // `decoded_token` is a `serde_json::value::Value` which looks like this:
                //
                // "user": String(
                //     "{\"name\":\"Wison Ye\",\"role\":\"Administrator\"}",
                // ),
                let user_string = decoded_token.get("user").unwrap().as_str().unwrap();
                let decoded_user: User = serde_json::from_str(user_string).unwrap();

                // Now, we got the decoded user, then we can attach to the `request.header.user`.
                // But `Request.append_header()` asks for a value of `impl ToHeaderValues` which
                // the `User` struct doesn't implement, that's why we have to use `String` as the
                // walk around.
                request.append_header("user", serde_json::to_string(&decoded_user).unwrap());
            }

            // If token is invalid, then rejct immediately
            None => {
                let reject_response = Response::builder(StatusCode::Unauthorized)
                    .header("Content-Type", "applicationx/json")
                    .body(serde_json::json!({
                        "errorCode": StatusCode::Unauthorized.to_string(),
                        "errorMessage": "Token is invalid."
                    }))
                    .build();

                return Ok(reject_response);
            }
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
    server.with(JwtAuthenticationMiddleware {});

    //
    // `/auth` to generate JWT token if login succeed
    //
    server.at("/auth").post(|mut req: Request<()>| async move {
        let mut res = Response::builder(StatusCode::Unauthorized)
            .header("Content-Type", "applicationx/json")
            .build();

        let post_body = req.body_json::<LoginUser>().await;
        println!("[ Auth Route ] - post_body: {:#?}", post_body);

        // If POST body is missing attributes
        if post_body.is_err() {
            res.set_body(serde_json::json!({
                "success": false,
                "errorCode": res.status().to_string(),
                "errorMessage": "'username' and 'password' is required."
            }));

            return Ok(res);
        }

        let login_user = post_body.unwrap();
        let login_success = login_user.username == "wison" && login_user.password == "demo";

        if login_success {
            let user = User {
                name: "Wison Ye".to_string(),
                role: "Administrator".to_string(),
            };
            res.set_status(StatusCode::Ok);
            res.set_body(serde_json::json!({
                "success": true,
                "token": jwt_util::generate_token(user)
            }));
        } else {
            res.set_body(serde_json::json!({
                "success": false,
                "errorCode": res.status().to_string(),
                "errorMessage": "'username' or 'password' is invalid."
            }));
        }

        Ok(res)
    });

    //
    // `/home` route which under JWT authentication protected
    //
    server.at("/home").get(|req: Request<()>| async move {
        let jwt_user: User = match req.header("user") {
            Some(decode_user_string) => {
                serde_json::from_str::<User>(decode_user_string.as_str()).unwrap()
            }
            None => unreachable!(),
        };
        println!("[ Home Route ] - jwt_user: {:#?}", &jwt_user);

        let res = Response::builder(StatusCode::Ok)
            .header("Content-Type", "applicationx/json")
            // .body(serde_json::to_value(jwt_user).unwrap())
            .body(serde_json::json!({
                "success": true,
                "dashboard": "Here is the Dashboard",
                "currentUser": serde_json::to_value(jwt_user).unwrap()
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

use serde_json;
use tide::prelude::{Deserialize, Serialize};
use tide::{Request, Server};

///
#[derive(Debug, Serialize, Deserialize)]
struct UserSettings {
    prefer_language: String,
    reload_when_changed: bool,
}

///
#[derive(Debug, Serialize, Deserialize)]
enum UserRole {
    Administrator,
    Operator,
    NormalUser,
}

///
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    role: UserRole,
    settings: Option<UserSettings>,
}

/// `Route.get()` accepts an `Async function`/`Future` that
/// take a `Request<State>` param and return an `impl Into<Response>`.
async fn get_default_user(_req: Request<()>) -> tide::Result<serde_json::value::Value> {
    let default_user = User {
        name: "Wison Ye".to_string(),
        role: UserRole::Administrator,
        settings: Some(UserSettings {
            prefer_language: "English".to_string(),
            reload_when_changed: true,
        }),
    };

    Ok(serde_json::to_value(default_user).unwrap())
}

///
async fn add_new_user(mut req: Request<()>) -> tide::Result<serde_json::value::Value> {
    #[derive(Serialize)]
    struct AddUserResult {
        success: bool,
        added_user: Option<User>,
        fail_reason: Option<String>,
    }

    let mut result = AddUserResult {
        success: true,
        added_user: None,
        fail_reason: None,
    };

    let added_user = req.body_json().await;
    println!("POST -> added_user: {:#?}", added_user);

    match added_user {
        Ok(user) => result.added_user = user,
        Err(error) => result.fail_reason = Some(error.to_string()),
    }

    Ok(serde_json::to_value(result).unwrap())
}

///
#[async_std::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();

    println!("[ Basic Server Demo ]\n");

    // Create new server without any shared `State`,
    // that's why the type is `Server<()>`
    let mut server: Server<()> = tide::new();

    // Routing setup
    server.at("get-default-user").get(get_default_user);
    server.at("add-user").post(add_new_user);

    // Start listening on specified address and port
    let listen_to = "0.0.0.0:8080";
    println!("Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;

    println!("Server is closed.");
    Ok(())
}

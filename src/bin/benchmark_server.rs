use serde_json;
use tide::prelude::{Deserialize, Serialize};

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

///
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("[ Benchmark Server Demo ]\n");

    let mut server = tide::new();

    // Default route
    server.at("").get(|_| async { Ok("Benchmark testing.") });

    // JSON route
    server.at("json-benchmark").get(|_| async {
        let default_user = User {
            name: "Wison Ye".to_string(),
            role: UserRole::Administrator,
            settings: Some(UserSettings {
                prefer_language: "English".to_string(),
                reload_when_changed: true,
            }),
        };

        tide::Result::Ok(serde_json::to_value(default_user).unwrap())
    });

    let listen_to = "0.0.0.0:8080";
    println!("Benchmark Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;

    println!("Server is closed.");
    Ok(())
}

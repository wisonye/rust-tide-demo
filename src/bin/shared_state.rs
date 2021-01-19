use std::sync::Arc;
// use tide::prelude::{Deserialize, Serialize};
use tide::Request;

///
#[derive(Debug)]
struct DeviceResponseSessions {
    unique_id: Option<String>,
    device_list: Vec<String>,
}

/// Because `tide` always clones the shared state, that's why it asks for 
/// implementing the `Clone` trait. If you don't want that to affect the 
/// performance, then you should wrap the instance inside the `Arc<T>`!!!
#[derive(Debug, Clone)]
struct ShareState {
    sessions: Arc<DeviceResponseSessions>,
}

///
impl DeviceResponseSessions {
    ///
    pub fn new() -> Self {
        let mut instance = Self {
            unique_id: None,
            device_list: vec!["111".to_string(), "222".to_string(), "333".to_string()],
        };

        // Use the unique instance pointer (memory address) as the unique id.
        instance.unique_id = Some(format!("Instance address as id: {:p}", &instance));

        instance
    }

    ///
    pub fn print_unique_id(&self) -> &str {
        self.unique_id.as_ref().unwrap()
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("[ Shared State Demo ]\n");

    let share_sessions = Arc::new(DeviceResponseSessions::new());
    let mut server = tide::with_state(ShareState {
        sessions: Arc::clone(&share_sessions),
    });

    server.at("/").get(|req: Request<ShareState>| async move {
        println!(
            "shared state sessions unique id: {}",
            req.state().sessions.print_unique_id()
        );

        Ok("Ok".to_string())
    });

    // Start listening on speficied address and port
    let listen_to = "0.0.0.0:8080";
    println!("Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;

    println!("Server is closed.");
    Ok(())
}

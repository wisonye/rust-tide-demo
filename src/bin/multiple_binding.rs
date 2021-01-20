use tide::listener::{ConcurrentListener, Listener};

///
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("[ Multiple Binding Demo ]\n");

    // Enable logging
    tide::log::start();

    // Create server
    let mut server = tide::new();

    // Sample hello get request handler
    server.at("/").get(|req: tide::Request<()>| async move {
        let response_message = format!(
            "Hi, I got your reuqest which is from '{}'",
            req.url().to_string()
        );
        Ok(response_message)
    });

    //
    // `Server.bind()` accepts the `impl ToListener<State>` and return
    // the `Listener` instance.
    //
    // `Vec<T>` and `String` already implemented the `ToListener` trait
    //
    let listen_targets = vec!["localhost:8080", "127.0.0.1:9000"];
    let mut listener: ConcurrentListener<()> = server.bind(listen_targets).await?;

    for info in listener.info().iter() {
        println!("Server is listening on: {}\n", info);
    }

    listener.accept().await?;

    // The `.bind()` and `.accept()` above actually can write as:
    // server.listen(listen_targets).await?;

    println!("Server is closed.");
    Ok(())
}

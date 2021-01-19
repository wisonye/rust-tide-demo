# `Tide` tutorial

Q: Why use `tide` as the `Rust` HTTP crate?

A: As it has unique features below:

- Based on `async-std`, simple, powerful and extremely fast.
- Combines some of the best ideas from frameworks like `Rocket`, `Actix`, and `Gotham`.

</br>

- ## Create and run the server

    ```rust
    // Create new server without any shared `State`,
    // that's why the type is `Server<()>`
    let mut server: tide::Server<()> = tide::new();

    // Start listening on speficied address and port
    let listen_to = "0.0.0.0:8080";
    println!("Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;
    ```

    </br>

- ## Request handler

    In `tide`, calling `Server.at(path: &str)` to create a `Route` instance.

    For the `Route` instance, it has a few methods to setup an `Endpoint` to handle
    the specified `Request` to the particular path:

    ```rust
    Route.get()
    Route.post()
    Route.put()
    Route.delete()
    ```

    All the methods above accept an `Async function`/`Future` that takes a 
    `Request<State>` param and return a `impl Into<Response>`. All the types below
    already implemented the `Into<Response>` trait, as the following implementations
    you can found inside the `Response` struct.

    ```rust
    impl<'a> From<&'a str> for Response
    impl From<tide::Body> for Response
    impl From<tide::Error> for Response
    impl From<tide::http::Response> for Response
    impl From<tide::StatusCode> for Response
    impl From<String> for Response
    impl From<serde_json::value::Value> for Response
    ```

    [`basic_server`](src/bin/basic_server.rs) is the demo to show how to setup request handler. You can run the 
    demo by running:

    ```bash
    cargo watch --exec "run --bin basic_server"
    ```

    And then run the command below to test it:

    ```bash
    curl --header "Content-Type: application/json" http://localhost:8080/get-default-user
    # {"name":"Wison Ye","role":"Administrator","settings":{"prefer_language":"English","reload_when_changed":true}}

    curl --header "Content-Type: application/json" \
        --data '{"name": "Wison Ye", "role": "Operator", "settings": {"prefer_language": "Chinese", "reload_when_changed": true}}' \
        http://localhost:8080/add-user
    # POST -> added_user: Ok(
    #     Some(
    #         User {
    #             name: "Wison Ye",
    #             role: Operator,
    #             settings: Some(
    #                 UserSettings {
    #                     prefer_language: "Chinese",
    #                     reload_when_changed: true,
    #                 },
    #             ),
    #         },
    #     ),
    # )

    curl --header "Content-Type: application/json" \
        --data '{"name": "Wison Ye", "role": "Operator", "settings": null}' \
        http://localhost:8080/add-user
    # POST -> added_user: Ok(
    #     Some(
    #         User {
    #             name: "Wison Ye",
    #             role: Operator,
    #             settings: None,
    #         },
    #     ),
    # )
    ```

    </br>

- ## Shared state

    If you want all `tide::Request` instances to access the shared (immutable) state, here
    is the way to do that:

    ```rust
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
    ```

    When you create the server, you should use `tide::with_state()` like below:

    ```rust
    let share_sessions = Arc::new(DeviceResponseSessions::new());
    let mut server = tide::with_state(ShareState {
        sessions: Arc::clone(&share_sessions),
    });

    server.at("/").get(|req: Request<ShareState>| async move {
        println!(
            "shared state unique id: {}",
            req.state().sessions.print_unique_id()
        );

        Ok("Ok".to_string())
    });
    ```

    [`shared_state`](src/bin/shared_state.rs) is the demo to show how to setup share state. You can run the 
    demo by running:

    ```bash
    cargo watch --exec "run --bin shared_state"
    ```

    And then run the command below to test it:

    ```bash
    curl localhost:8080
    curl localhost:8080
    curl localhost:8080
    curl localhost:8080
    ```

    No matter how many times you try, it should always print out the same `DeviceResponseSessions`
    instance address (as the unique id value) like below:

    ```bash
    # shared state sessions unique id: Instance address as id: 0x7ffca9e8e9e8
    # shared state sessions unique id: Instance address as id: 0x7ffca9e8e9e8
    # shared state sessions unique id: Instance address as id: 0x7ffca9e8e9e8
    # shared state sessions unique id: Instance address as id: 0x7ffca9e8e9e8
    # shared state sessions unique id: Instance address as id: 0x7ffca9e8e9e8
    ```

    </br>

- ## Serve static files

    ```rust
    // Serve static files
    let mut static_images_serve_route = server.at("/images");
    let _ = static_images_serve_route.serve_dir("images/");
    ```

    [`serve_static_files`](src/bin/serve_static_files.rs) is the demo to show how to setup static files serving. You can run the 
    demo by running:

    ```bash
    cargo watch --exec "run --bin serve_static_files"
    ```

    And then open the below url in your browser to view the sample image:

    ```bash
    http://localhost:8080/images/preview-4.png
    ```
    </br>

- ## Listen to multiple addresses

    </br>


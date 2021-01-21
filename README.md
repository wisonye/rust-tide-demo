# `Tide` tutorial

Q: Why use `tide` as the `Rust` HTTP crate?

A: As it has unique features below:

- Based on `async-std`, simple, powerful and extremely fast.
- Combines some of the best ideas from frameworks like `Rocket`, `Actix`, and `Gotham`.

</br>

## 1. Create and run the server

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

## 2. Request handler

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

## 3. Shared state

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

## 4. Serve static files

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

## 5. Listen to multiple addresses
    
`Server.bind()` accepts an `impl ToListener<State>` and return the `impl Listener<State>` instance.

So you can use  `Server.bind()` to bind multiple targets before calling `Listener.accept()`. All types
can be passed into `Server.bind()` must implement the `ToListener` trait. By default, all types below
already implemented the `ToListener` trait:

```rust
String
&str
Vec<String>
Vec<&str>
```

But you're not easy to implement the `ToListener` trait for your own struct, as the `ParsedListener` is
private. The sample below won't be compiled:

```rust
///
struct LocalBindingTargetA(u32);
//
impl<State> tide::listener::ToListener<State> for LocalBindingTargetA
where
    State: Clone + Send + Sync + 'static,
{
    type Listener = tide::listener::ParsedListener<State>;
    //
    fn to_listener(self) -> async_std::io::Result<Self::Listener> {
        tide::listener::ToListener::<State>::to_listener(format!("localhost::{}", self.0))
    }
}
```

The `Server.listen()` just a shortcut to call `Server.bind().accept()`.

That means `server.listen(listen_targets).await?;` can be written as:

```rust
let listen_targets = vec!["localhost:8080", "127.0.0.1:9000"];
let mut listener: ConcurrentListener<()> = server.bind(listen_targets).await?;
listener.accept().await?;
```

[`multiple_binding`](src/bin/multiple_binding.rs) is the demo to show how to setup multiple bindings. 
You can run the demo by running:

```bash
cargo watch --exec "run --bin multiple_binding"
```

After that, you should see the console log llike below:

```bash
[ Multiple Binding Demo ]

tide::log Logger started
    level Info
Server is listening on: http://[::1]:8080

Server is listening on: http://127.0.0.1:9000
```

Then, you can run the command below to test it:

```bash
curl localhost:8080
curl localhost:9000
```

</br>

## 6. Dealing with `Status code`

Usually, you got several approaches to attach the specified `status code` to the `Response`:

```rust
# Create `Response` with the `StatusCode`
let response = Response::new(StatusCode::Ok);
let response = Response::new(StatusCode::Unauthorized);
let response = Response::new(StatusCode::NotFound);
let response = Response::new(StatusCode::BadRequest);
let response = Response::new(StatusCode::InternalServerError);


# Reset after created
let mut response = Response::new(StatusCode::Ok);
response.set_status(404);
response.set_status(StatusCode::InternalServerError);


# Create with `ResponseBuilder`
Response::builder(StatusCode::Unauthorized)
    .header("Content-Type", "application/json")
    .body(serde_json::json!({
        "success": status_code.is_success(),
        "status_code": status_code.to_string()
    }))
    .build()
```

[`status_code`](src/bin/status_code.rs) is the demo to show how to setup status code.
You can run the demo by running:

```bash
cargo watch --exec "run --bin status_code"
```

Then you can run commands below to test it:

```bash
curl localhost:8080/status-code-test-1
# {"status_code":"401","success":false}

curl localhost:8080/status-code-test-2
# {"status_code":"400","success":false}

curl localhost:8080/status-code-test-3
# {"status_code":"404","success":false}

curl localhost:8080/status-code-test-4
# {"status_code":"500","success":false}
```

</br>

## 7.Middleware in `tide`

- What `middleware` in `tide` can do:

    - Do something or modify the request before going forward to the next middleware or request handler.
    For example:
        - Logging each original request
        - Do request data checking
        - Query identification from the database and replace the request data fields

        </br>

    - Do something after finishing the request handler. For example:
        - Logging after request handler has been done: Processing time cost, etc.
        - Post-processing requirement: failure report, third-party integration API call after handling succeed.

        </br>

    - Authentication mechanism with the earlier rejected feature:
        - JWT
        - Another `token-based` authentication

        </br>

    </br>

- How it works?

    - Implement the `tide::Middleware` trait:

        The `handle` async function can either continue processing by calling `Ok(next.run(request).await)` or 
        immediately return a response.

        ```rust
        ///
        struct TokenBasedAuthenticationMiddleware {}

        ///
        #[async_trait::async_trait]
        impl<State> tide::Middleware<State> for TokenBasedAuthenticationMiddleware
        where State: Clone + Send + Sync + 'static,
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
        ```

        </br>

    - Add you own middleware to the `Server`:

        Middleware can only be added at the "top level" of an application, and is processed in 
        the order in which it is applied.

        ```rust
        // Apply the middleware
        server.with(TokenBasedAuthenticationMiddleware {});
        ```

        </br>

    That's it, middleware in `tide` is so simple to deal with.

    [`middleware`](src/bin/middleware.rs) is the demo to show how to implement the token-based
    authentication middleware.  You can run the demo by running:
    
    ```bash
    cargo watch --exec "run --bin middleware"
    ```
    
    Then you can run commands below to test it:
    
    ```bash
    curl --header "Authorization: demo-token" localhost:8080/home
    # {"user":{"name":"Wison","role":"Administrator"}}
    
    curl --header "Authorization: wrong_token" localhost:8080/home
    # {"errorCode":"401","errorMessage":"Token is invalid."}
    
    curl localhost:8080/home
    # {"errorCode":"401","errorMessage":"Token is invalid."}
    ```

    </br>

## 8. JWT Authentication middleware

[`jwt_middleware`](src/bin/jwt_middleware.rs) is the demo to show how to implement the JWT
authentication middleware and API based on that.  You can run the demo by running:

```bash
cargo watch --exec "run --bin jwt_rest_api"
```

Then you can run commands below to test it:

```bash
# Login fail
curl --request POST localhost:8080/auth
# {"errorMessage":"'username' and 'password' is required.","success":false}

# Login fail
curl --data '{ "username": "xxx", "password": "demo" }' localhost:8080/auth
# {"errorMessage":"'username' or 'password' is invalid.","success":false}

# Login correctly and get back the JWT token
curl --data '{ "username": "wison", "password": "demo" }' localhost:8080/auth
# {"success":true,"token":"eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjoie1wibmFtZVwiOlwiV2lzb24gWWVcIixcInJvbGVcIjpcIkFkbWluaXN0cmF0b3JcIn0ifQ.Sf0kbnrP9AwQfd4YmeCt029PrMCEQAsqScizvWvYEpo"}


# `/home` without token should fail
curl localhost:8080/home
{"errorCode":"401","errorMessage":"Token is invalid."}

# `/home` with wrong token should fail
curl --header "Authorization: jwt xxxx" localhost:8080/home
{"errorCode":"401","errorMessage":"Token is invalid."}


# `/home` with correct token should success
curl --header "Authorization: eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjoie1wibmFtZVwiOlwiV2lzb24gWWVcIixcInJvbGVcIjpcIkFkbWluaXN0cmF0b3JcIn0ifQ.Sf0kbnrP9AwQfd4YmeCt029PrMCEQAsqScizvWvYEpo" localhost:8080/home

# tide::log::middleware <-- Request received
#     method GET
#     path /home
# [ Middleware trait --> handle ] - url path: /home
# [ Middleware trai --> handle ] - token_str: eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjoie1wibmFtZVwiOlwiV2lzb24gWWVcIixcInJvbGVcIjpcIkFkbWluaXN0cmF0b3JcIn0ifQ.Sf0kbnrP9AwQfd4YmeCt029PrMCEQAsqScizvWvYEpo
# [ JwtUtil --> verfiy_token, decoded_value: Object({
#     "user": String(
#         "{\"name\":\"Wison Ye\",\"role\":\"Administrator\"}",
#     ),
# })
# [ Home Route ] - jwt_user: User {
#     name: "Wison Ye",
#     role: "Administrator",
# }
# tide::log::middleware --> Response sent
#     method GET
#     path /home
#     status 200 - OK
#     duration 289.824µs
#
# {"currentUser":{"name":"Wison Ye","role":"Administrator"},"dashboard":"Here is the Dashboard","success":true}
```
</br>


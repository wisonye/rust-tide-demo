#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("[ Serve Static Files Demo ]\n");

    tide::log::start();

    let mut server = tide::new();

    // Serve static files
    let mut static_images_serve_route = server.at("/images");
    let _ = static_images_serve_route.serve_dir("images/");

    // Start listening on speficied address and port
    let listen_to = "0.0.0.0:8080";
    println!("Server is listening on: {}\n", listen_to);
    server.listen(listen_to).await?;

    println!("Server is closed.");
    Ok(())
}

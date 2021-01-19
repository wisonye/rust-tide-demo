use tide::{Request, Server};

#[async_std::main]
async fn main() -> tide::Result<()> {
     let server = tide::new();
 
     // Static folder (files) serve
     let mut static_images_serve_route = server.at("images");
     static_images_serve_route.serve_dir("~/Screenshots");
 
     // Start listening on speficied address and port
     let listen_to = "0.0.0.0:8080";
     println!("Server is listening on: {}\n", listen_to);
     server.listen(listen_to).await?;
 
     println!("Server is closed.");
    Ok(())
}

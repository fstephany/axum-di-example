use axum::Server;
use di_examples::{struct_static, trait_static, trait_static};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting demos");
    let socket_9090 = "127.0.0.1:9090".parse()?;
    let socket_9091 = "127.0.0.1:9091".parse()?;
    let socket_9092 = "127.0.0.1:9092".parse()?;

    let _ = tokio::join!(
        Server::bind(&socket_9090).serve(struct_static::build_router().into_make_service()),
        Server::bind(&socket_9091).serve(trait_static::build_router().into_make_service()),
        Server::bind(&socket_9092).serve(dynamic::build_router().into_make_service())
    );

    Ok(())
}

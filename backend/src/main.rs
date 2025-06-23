use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(root));

    // Run it
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Axum!"
}
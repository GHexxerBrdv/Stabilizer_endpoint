use axum::{
    Router,
    routing::get
};

#[tokio::main]
async fn main() {
    println!("Welcome to the Stabilzer server");

    let app = Router::new().route("/ok", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
// router is responsible for setting up routes for the application
// routing::get is responsible for the get requests, routing::{get, post, put, delete, patch} can handle many requests operations
use axum::{
    Router,
    extract::{Json, Path},
    http::StatusCode,
    routing::get,
};

mod models;
mod utils;

// fn main should not be possible to be async, but because of the tokio crate we can make it async with the macro above it
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on {addr:?}");
    axum::serve(listener, route()).await.unwrap();
}

fn route() -> Router {
    Router::new().route("/auth/login", get(login_get).post(login_post)) // Post requests can be handled with the .post() method
}

async fn login_get() -> String {
    "Logged in GET".to_owned()
}

async fn login_post() -> String {
    "Logged in POST".to_owned()
}

async fn cards_get() -> String {
    "Cards in GET".to_owned()
}

async fn cards_post() -> String {
    "Cards in POST".to_owned()
}

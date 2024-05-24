use axum::{
    extract::{Path, Query}, response::IntoResponse, routing::{get, post}, Json, Router
};
use rust_test::mods;
use serde::Deserialize;
use serde_json::json;

async fn handle_add(a: i32, b: i32) -> impl IntoResponse {
    let res: i32 = mods::add(a, b);
    format!("{} + {} = {}", a, b, res)
}

#[derive(Debug, Deserialize)]
struct Name {
    name: String,
}

async fn handle_param(Query(params): Query<Name>) -> impl IntoResponse {
    format!("Hello, {}!", params.name)
}

#[derive(Debug, Deserialize)]
struct Multiply {
    a: i32,
    b: i32,
}

async fn handle_multiply(Json(params): Json<Multiply>) -> impl IntoResponse {
    let res: i32 = mods::multiply(params.a, params.b);
    Json(json!({"result": res}))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .merge(get_routes())
        .merge(post_routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


fn get_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/add/:a/:b", get(|Path((a, b)): Path<(i32, i32)>| async move {
            handle_add(a, b).await
        }))
        .route("/params", get(handle_param))
}

fn post_routes() -> Router {
    Router::new()
        .route("/multiply", post(handle_multiply))
}
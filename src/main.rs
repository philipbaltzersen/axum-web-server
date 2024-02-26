use axum::{
    body::Body,
    http::StatusCode,
    extract::{Path, Query},
    response::{IntoResponse, Response},
    routing::{get, post}, 
    Json, Router,
};
use serde::{Deserialize, Serialize};

// Struct for query parameters
#[derive(Deserialize)]
struct Page {
    number: u32,
}

// Struct for the JSON body
#[derive(Deserialize)]
struct Item {
    title: String,
}

// Handler to demonstrate path and query extractor
async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Handler for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created"))
        .unwrap()
    }

// Handler for /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Philip".to_string(),
            email: "philip@email.com".to_string(),
        },
        User {
            id: 2,
            name: "Per".to_string(),
            email: "per@email.com".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(add_item))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

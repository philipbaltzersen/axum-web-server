use axum::{
    body::Body,
    http::StatusCode,
    extract::{Path, Query},
    response::{IntoResponse, Response},
    routing::{get, post, delete}, 
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

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

// Handler for deleting user that might return an error
async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Deleted user".into(),
            email: "Deleted email".into(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

// Function that would delete user
async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    // Simulate error for demo
    if user_id == 1 {
        Err("User cannot be deleted".to_string())
    } else {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // TODO: Use environment variables to populate parameters
    let database_url = "mysql://<<USERNAME>>:<<PASSWORD>>@<<HOSTNAME>>/<<DATABASE NAME>>";
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(add_item))
        .route("/create-user", post(create_user))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/users", get(list_users));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

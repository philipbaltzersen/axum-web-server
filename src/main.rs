use axum::{
    body::Body,
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, delete, post}, 
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{MySqlPool, Row};


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
async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
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
        .route("/create-user", post(create_user))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/users", get(get_users))
        .layer(Extension(pool));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Cargo.toml
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
// serde = { version = "1", features = ["derive"] }

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Equipe {
    id: i32,
    nom: String,
}

#[derive(Debug, Deserialize)]
struct CreateEquipe {
    nom: String,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://user:password@localhost/apirust"; // Replace with your DB URL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to Postgres.");

    let app = Router::new()
       // .route("/equipe", get(get_users).post(create_user))
        .route("/equipe/:id", get(get_user_by_id))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as!(Equipe, "SELECT id, nom FROM Equipe")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(users)
}

pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let user = sqlx::query_as!(Equipe, "SELECT id, nom FROM equipe WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(user) = user {
        Ok(Json(user))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
/* 
async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(user))
}*/
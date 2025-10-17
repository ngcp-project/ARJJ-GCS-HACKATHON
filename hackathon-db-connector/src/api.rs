use axum::{routing::{get, post}, Router};
use sqlx::postgres::PgPool;
use sqlx::Row;
use std::net::SocketAddr;
use axum::extract::State;
use serde::{Serialize, Deserialize};
use axum::Json;
use axum::extract::Json as AxumJson;
use tower_http::cors::{CorsLayer, Any};
use axum::http::{Method, header};

#[derive(Serialize, sqlx::FromRow)]
struct Program {
    id: i32,
    name: String,
    description: String,
    image_url: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    barcode: i64,
}

async fn get_programs(State(pool): State<PgPool>) -> Json<Vec<Program>> {
    let programs = sqlx::query_as::<_, Program>(
        "SELECT id, name, description, image_url FROM programs",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(programs)
}

async fn login(State(pool): State<PgPool>, AxumJson(payload): AxumJson<LoginRequest>) -> Json<Option<LoginResponse>> {
    println!("Login attempt for username='{}'", payload.username);

    let result = sqlx::query(
        "SELECT barcode FROM users WHERE username = $1 AND password = $2"
    )
    .bind(&payload.username)
    .bind(&payload.password)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(opt_row) => {
            if let Some(r) = opt_row {
                let barcode: i64 = r.get::<i64, _>("barcode");
                println!("Login successful for '{}' -> barcode={}", payload.username, barcode);
                Json(Some(LoginResponse { barcode }))
            } else {
                println!("Login failed for '{}': invalid credentials", payload.username);
                Json(None)
            }
        }
        Err(e) => {
            println!("DB error during login for '{}': {:?}", payload.username, e);
            // Return None to keep the same response shape; frontend will treat as failed login
            Json(None)
        }
    }
}

pub async fn start_server(pool: PgPool) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/programs", get(get_programs))
        .route("/login", post(login))
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
use axum::{extract::{Path, State}, http::StatusCode, routing::{get,post}, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};


async fn get_key(State(pool): State<PgPool>, Path(key): Path<String>) -> Result<String, StatusCode>{
    let value: Option<String> = sqlx::query_scalar("SELECT value FROM kv WHERE key = $1")
        .bind(&key)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    value.ok_or(StatusCode::NOT_FOUND)

}

async fn set_key(State(pool): State<PgPool>, body: String) ->Result<&'static str, StatusCode> {
    let (key, value) = body.split_once('=').ok_or(StatusCode::BAD_REQUEST)?;
    if key.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query(
        "INSERT INTO kv (key, value) VALUES ($1, $2) \
            ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value"
    )
        .bind(key)
        .bind(value)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
     
        Ok("OK")
}

#[tokio::main]

async fn main() {
    let url = std::env::var("DATABASE_URL"). expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("failed to connect to database");



    sqlx::query("CREATE TABLE IF NOT EXISTS kv (key TEXT PRIMARY KEY, value TEXT NOT NULL)")
        .execute(&pool)
        .await
        .expect("failed to create table");
    
    let app = Router::new()
        .route("/get/{key}", get(get_key))
        .route("/set", post(set_key))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();

}

use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Router};
use sqlx::PgPool;

pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE TABLE IF NOT EXISTS kv (key TEXT PRIMARY KEY, value TEXT NOT NULL)")
        .execute(pool)
        .await?;
    Ok(())
}


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
async fn delete_key(State(pool): State<PgPool>, Path(key): Path<String>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM kv WHERE key = $1")
        .bind(&key)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/get/{key}", get(get_key))
        .route("/set", post(set_key))
        .route("/delete/{key}", delete(delete_key))
        .with_state(pool)
}

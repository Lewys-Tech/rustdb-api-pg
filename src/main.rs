use sqlx::postgres::PgPoolOptions;

#[tokio::main]

async fn main() -> Result<(), sqlx::Error> {
    let url = std::env::var("DATABASE_URL"). expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .wait?;

    let row: (i32, ) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;
}

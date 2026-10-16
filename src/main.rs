use sqlx::postgres::PgPoolOptions;


#[tokio::main]

async fn main() {
    let url = std::env::var("DATABASE_URL"). expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("failed to connect to database");

    rustdb_api_pg::init_db(&pool).await.expect("failed to create table");


    let app = rustdb_api_pg::app(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();

}

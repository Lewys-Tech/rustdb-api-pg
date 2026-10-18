use axum::{body::Body, http::{Request, StatusCode}, Router};
use http_body_util::BodyExt;
use sqlx::PgPool;
use tower::ServiceExt;


async fn send(app: &Router, req: Request<Body>) -> (StatusCode, String) {
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(bytes.to_vec()).unwrap())


}

#[sqlx::test]
async fn get_missing_key_returns_404(pool: PgPool) {
    rustdb_api_pg::init_db(&pool).await.unwrap();
    let app = rustdb_api_pg::app(pool);

    let (status, _) = send(&app, Request::get("/get/nothing").body(Body::empty()).unwrap()).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

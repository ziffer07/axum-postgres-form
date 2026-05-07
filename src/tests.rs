use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::{AppState, create_router, database_connection};





#[tokio::test]
async fn test_main_page() {

    let pool = database_connection().await;

    let state = AppState {
        connection_pool: pool,
    };

    let response = create_router(state)
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"<h1>Home Page</h1>"#));
}


#[tokio::test]
async fn test_form_page() {

    let pool = database_connection().await;

    let state = AppState {
        connection_pool: pool,
    };

    let response = create_router(state)
        .oneshot(Request::builder().uri("/form-page").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"<form action="/form-page" method="post">"#));
}
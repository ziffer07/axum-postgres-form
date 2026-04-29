use std::{str::FromStr, time::Duration};

use axum::{Form, Router, extract::State, response::{Html, IntoResponse, Response}, routing::get};
use askama::Template;
use serde::{Deserialize};
use sqlx::{ConnectOptions, PgPool, postgres::{PgConnectOptions, PgPoolOptions}};


struct HtmlTemplate<T> (T);
impl <T> IntoResponse for HtmlTemplate<T>
where 
    T: Template,
{  
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => "Internal Server Error".into_response(),
        }
    }
}


#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate{}

#[derive(Template)]
#[template(path = "form-page.html")]
struct FormPageTemplate{
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}




async fn home_page_handler() -> impl IntoResponse {
    let template = HomeTemplate{};
    HtmlTemplate(template)
}

async fn form_page_handler() -> impl IntoResponse {
    let template = FormPageTemplate{
        name: String::new(),
        email: String::new(),
    };
    HtmlTemplate(template)
}

async fn post_form_handler(State(app_state): State<AppState>, Form(data_fields): Form<FormData>) -> impl IntoResponse {
    let template = FormPageTemplate{
        name: data_fields.name.clone(),
        email: data_fields.email.clone(),
    };

    // let _ = add_name_email_to_db(
    //     &app_state.connection_pool,
    //     &data_fields.name, 
    //     &data_fields.email
    // ).await;

    match add_name_email_to_db(
        &app_state.connection_pool,
        &data_fields.name, 
        &data_fields.email
    ).await {
        Ok(_) => {},
        Err(e) => eprintln!("Database error: {}", e),
    }
    HtmlTemplate(template)
}



fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home_page_handler))
        .route("/form-page", get(form_page_handler).post(post_form_handler))
        .with_state(state)
}


// Database related code
#[derive(Clone)]
struct AppState {
    connection_pool: PgPool,
}

async fn database_connection() -> PgPool {
    let db_url = dotenvy::var("DATABASE_URL").expect(" Failed to connect to database");

    let options = PgConnectOptions::from_str(&db_url)
        .expect("failed to parse url")
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");

    pg_pool
}

// Query into database tables
async fn add_name_email_to_db(pool: &PgPool, name: &str, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO contents (name, email) VALUES ($1, $2)",
        name,
        email
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let pool = database_connection().await;
    let state = AppState {
        connection_pool: pool,
    };
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

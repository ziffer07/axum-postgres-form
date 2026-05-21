use std::{path::Path, str::FromStr, time::{Duration, SystemTime}};

use axum::{Router, body::Body, extract::{State, multipart::{Multipart}}, http::StatusCode, response::{Html, IntoResponse, Redirect, Response}, routing::get};
use askama::Template;
use serde::{Deserialize};
use sqlx::{ConnectOptions, PgPool, postgres::{PgConnectOptions, PgPoolOptions}};
use thiserror::Error;
use tower_http::services::ServeDir;


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

// These are all the templates needed. One template represents one page in .html file ---------------------------------------------------//
#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate{
    form_data: Vec<FormData>,
}

#[derive(Template)]
#[template(path = "form-page.html")]
struct FormPageTemplate{}

#[derive(Template)]
#[template(path = "server-error.html")]
struct ServerErrorTemplate{}

// ----------------------------------------------------------------------------------------------------------------------------------------//

// This struct has data that we are going to store in our database -----------------------------------------------------------------------//
#[derive(sqlx::FromRow, Deserialize)]
struct FormData {
    name: String,
    email: String,
    title: String,
    description: String,
    image_path: Option<String>
}
// ---------------------------------------------------------------------------------------------------------------------------------------//



// Blow are all the handlers that we need to post a form, store in database and display stuff from db on frontend ------------------------//

async fn home_page_handler(State(app_state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let form_data = get_all_data(&app_state.connection_pool).await?;
    let template = HomeTemplate{
        form_data,
    };
    Ok(HtmlTemplate(template))
}

async fn form_page_handler() -> impl IntoResponse {
    let template = FormPageTemplate{};
    HtmlTemplate(template)
}


async fn post_form_handler(
    State(app_state): State<AppState>,
    mut multipart: Multipart
) -> Response {
    let mut name = String::new();
    let mut email = String::new();
    let mut title = String::new();
    let mut description = String::new();
    let mut image_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        match field.name().unwrap_or("") {
            "name" => name = field.text().await.unwrap_or_default(),
            "email" => email = field.text().await.unwrap_or_default(),
            "title" => title = field.text().await.unwrap_or_default(),
            "description" => description = field.text().await.unwrap_or_default(),
            "image" => {
                let file_name = field.file_name()
                    .filter(|n| !n.is_empty())
                    .map(|n| n.to_string());

                if let Some(name) = file_name {
                    let bytes = field.bytes().await.unwrap_or_default();
                    if !bytes.is_empty() {
                        let save_path = format!("uploads/{}", name);
                        tokio::fs::create_dir_all("uploads").await.ok();
                        tokio::fs::write(&save_path, &bytes).await.ok();
                        image_path = Some(save_path);
                    }
                }
            }
            _ => {}
        }
    }
     match add_name_email_to_db(
        &app_state.connection_pool,
        &name, &email, &title, &description,
        image_path.as_deref(),
    ).await {
        Ok(_) => {},
        Err(e) => eprintln!("Database error: {}", e),
    }

    Redirect::to("/").into_response()
}


fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home_page_handler))
        .route("/form-page", get(form_page_handler).post(post_form_handler))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .with_state(state)
}


// Database related logic code ---------------------------------------------------------------------------------------------------------------------------//
#[derive(Clone)]
pub struct AppState {
    connection_pool: PgPool,
}

pub async fn database_connection() -> PgPool {
    let db_url = dotenvy::var("DATABASE_URL").expect(" Failed to connect to database");

    let options = PgConnectOptions::from_str(&db_url)
        .expect("failed to parse url")
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(10)
        .min_connections(1)
        // Drop connections idle longer than 10 minutes
        .idle_timeout(Duration::from_secs(600))
        // Recycle connections before the server closes them
        .max_lifetime(Duration::from_secs(1800))
        // Test connection health before handing it out
        .test_before_acquire(true)
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
async fn add_name_email_to_db(
    pool: &PgPool,
    name: &str,
    email: &str,
    title: &str,
    description: &str,
    image_path: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r"INSERT INTO contents (name, email, title, description, image_path) VALUES ($1, $2, $3, $4, $5)",
        name,
        email,
        title,
        description,
        image_path,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_all_data(pool: &PgPool) -> Result<Vec<FormData>, sqlx::Error> {
    let form_data: Vec<FormData> = sqlx::query_as(
        "SELECT name, email, title, description, regexp_replace(image_path, '^uploads/', '') AS image_path FROM contents ORDER BY contents.id DESC"
    ).fetch_all(pool).await?;

    Ok(form_data)
}

// -----------------------------------------------------------------------------------------------------------------------------------------------//


// Main connection --------------------------------------------------------------------------------------------------------------------------------//

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

// -----------------------------------------------------------------------------------------------------------------------------------------------//


// Errors in logic ------------------------------------------------------------------------------------------------------------------------------//

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Template error")]
    Template(#[from] askama::Error),
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response<Body> {
        let (status, response) = match self {
            AppError::Database(data_error) => server_error(data_error.to_string()),
            AppError::Template(error) => server_error(error.to_string()),
        };

        (status, response).into_response()
    }
}

fn server_error(_e: String) -> (StatusCode, Response<Body>) {

    let html_string = ServerErrorTemplate{}.render().unwrap();
    (StatusCode::INTERNAL_SERVER_ERROR, Html(html_string).into_response())
}

// -----------------------------------------------------------------------------------------------------------------------------------------------//

#[cfg(test)]
mod tests;
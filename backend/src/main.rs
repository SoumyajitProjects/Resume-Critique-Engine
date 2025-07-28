mod handlers;
mod models;
mod db;
mod auth;
mod config;

use warp::Filter;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Arc::new(config::Config::from_env());
    let db_pool = Arc::new(db::create_pool(&config.database_url).await.expect("Failed to create database pool"));

    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);

    // Routes
    let upload_route = warp::path("upload-resume")
        .and(warp::post())
        .and(warp::multipart::form().max_length(config.max_file_size))
        .and(with_db(db_pool.clone()))
        .and(with_config(config.clone()))
        .and_then(handlers::upload_resume);

    let critique_route = warp::path!("get-critique" / i32)
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and(with_config(config.clone()))
        .and_then(handlers::get_critique);

    let auth_routes = warp::path("auth")
        .and(
            warp::path("login")
                .and(warp::post())
                .and(warp::body::json())
                .and(with_db(db_pool.clone()))
                .and(with_config(config.clone()))
                .and_then(handlers::login)
                .or(
                    warp::path("register")
                        .and(warp::post())
                        .and(warp::body::json())
                        .and(with_db(db_pool.clone()))
                        .and(with_config(config.clone()))
                        .and_then(handlers::register)
                )
                .or(
                    warp::path("me")
                        .and(warp::get())
                        .and(auth::with_auth(config.clone()))
                        .and_then(handlers::get_current_user)
                )
        );

    let history_route = warp::path("history")
        .and(warp::get())
        .and(auth::with_auth(config.clone()))
        .and(with_db(db_pool.clone()))
        .and_then(handlers::get_history);

    let routes = upload_route
        .or(critique_route)
        .or(auth_routes)
        .or(history_route)
        .with(cors)
        .recover(handlers::handle_rejection)
        .with(warp::log("resume-critique-backend"));

    println!("Server starting on http://localhost:3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

fn with_db(db_pool: Arc<sqlx::PgPool>) -> impl Filter<Extract = (Arc<sqlx::PgPool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn with_config(config: Arc<config::Config>) -> impl Filter<Extract = (Arc<config::Config>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

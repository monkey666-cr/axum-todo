use std::io;

use axum::{routing::get, Router};
use deadpool_postgres::Runtime;
use dotenv::dotenv;

mod config;
mod db;
mod error;
mod form;
mod handler;
mod model;
mod response;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

use model::AppState;
pub use response::Response;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum-todo=debug");
    }

    tracing_subscriber::fmt().with_writer(io::stdout).init();

    // 解析.env文件
    dotenv().ok();

    let cfg = config::Config::from_env().expect("msg初始化配置失败");

    let pool = cfg
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .expect("初始化PGSQL失败");

    let app = Router::new()
        .route("/", get(handler::usage::usage))
        .route(
            "/todo",
            get(handler::todo_list::all).post(handler::todo_list::create),
        )
        .route(
            "/todo/:id",
            get(handler::todo_list::find)
                .put(handler::todo_list::update)
                .delete(handler::todo_list::delete),
        )
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind(&cfg.web.addr).await.unwrap();

    tracing::info!("服务器监听于: {}", &cfg.web.addr);

    axum::serve(listener, app).await.unwrap();
}

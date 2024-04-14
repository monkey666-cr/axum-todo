use axum::{routing::get, Router};
use dotenv::dotenv;

mod config;
mod error;
mod handler;
mod response;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

pub use response::Response;

#[tokio::main]
async fn main() {
    // 解析.env文件
    dotenv().ok();

    let app = Router::new().route("/", get(handler::usage));

    let cfg = config::Config::from_env().expect("msg初始化配置失败");

    let listener = tokio::net::TcpListener::bind(&cfg.web.addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

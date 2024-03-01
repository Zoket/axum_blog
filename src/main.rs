use std::sync::Arc;

use axum::{
    extract::{extractor_middleware, Extension},
    Router,
};
use blog::{
    config,
    handler::{backend, frontend},
    middleware, AppState,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "blog=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().expect("初始化配置失败");
    let pool = cfg
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("创建数据库连接池失败");

    let frontend_routers = frontend::router();
    let backend_routers = backend::router().layer(extractor_middleware::<middleware::Auth>());

    let app = Router::new()
        .nest("/", frontend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(Arc::new(AppState { pool })));

    tracing::info!("服务已启动");

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

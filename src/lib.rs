pub mod db;
pub mod error;
pub mod form;
pub mod handler;
pub mod model;
pub mod view;
pub mod config;
pub mod md;
pub mod password;
pub mod middleware;
pub mod cookie;


pub type Result<T> = std::result::Result<T, error::AppError>;

pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}

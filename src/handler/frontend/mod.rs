use axum::{routing::get, Router};
use serde::Deserialize;

use super::auth;

pub mod index;
pub mod topic;
pub fn router() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/auth", get(auth::login_ui).post(auth::login))
        .route("logout", get(auth::logout))
        .route("/category/:id", get(topic::list))
        .route("/topic/:id", get(topic::detail))
        .route("/archive/:dt", get(topic::archive))
}

#[derive(Deserialize)]
pub struct Args {
    pub page : Option<u32>,
}
impl Args {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
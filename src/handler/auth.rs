use std::sync::Arc;

use axum::extract::{Extension, Form};

use crate::{
    db::admin,
    error::{AppError, AppErrorType},
    form::AdminLogin,
    handler::{get_client, redirect_with_cookie},
    password,
    view::auth::Login,
    AppState, Result,
};

use super::{log_error, render, HtmlView, RedirectView};

pub async fn login_ui() -> Result<HtmlView> {
    let handler_name = "auth/login_ui";
    let tmpl = Login {};
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<AdminLogin>,
) -> Result<RedirectView> {
    let handler_name = "auth/login";
    tracing::debug!("{}", password::hash("axum.rs")?);
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let admin_info = admin::find(&client, &frm.email)
        .await
        .map_err(|err| match err.types {
            AppErrorType::NotFound => AppError::incorrect_login(),
            _ => err,
        })
        .map_err(log_error(handler_name))?;

    let verify =
        password::verify(&frm.password, &admin_info.password).map_err(log_error(handler_name))?;
    if !verify {
        return Err(AppError::incorrect_login());
    }
    redirect_with_cookie("/admin", Some(&admin_info.email))
}

pub async fn logout() -> Result<RedirectView> {
    redirect_with_cookie("/", Some(""))
}
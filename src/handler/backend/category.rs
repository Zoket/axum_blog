use std::sync::Arc;

use axum::extract::{Extension, Form, Path, Query};

use crate::{
    db::category, form::{self, EditCategory}, handler::{get_client, log_error, redirect, render, HtmlView, RedirectView}, view::backend::category::{Add, Edit, Index}, AppState, Result
};

use super::Args;

// 添加分类UI
pub async fn add_ui() -> Result<HtmlView> {
    let handler_name = "backend/category/add_ui";
    let tmpl = Add {};
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::CreateCategory>,
) -> Result<RedirectView> {
    let handler_name = "backend/category/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::create(&client, &frm)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=分类添加成功")
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let handler_name = "backend/category/index";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = Index {
        list,
        msg: args.msg,
    };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<RedirectView> {
    let handler_name = "backend/category/del";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::del_or_restore(&client, id, true)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=分类删除成功")
}

pub async fn edit_ui(
    Extension(state):Extension<Arc<AppState>>,
    Path(id):Path<i32>,
) -> Result<HtmlView> {
    let handler_name = "backend/category/edit_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let item = category::find(&client, id).await.map_err(log_error(handler_name))?;
    let tmpl = Edit { item };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn edit(
    Extension(state):Extension<Arc<AppState>>,
    Form(frm):Form<EditCategory>,
)->Result<RedirectView> {
    let handler_name = "backend/category/edit";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::edit(&client, &frm).await.map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=分类修改成功")
}
use std::fmt::Display;

use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Template,
    NotFound,
    Duplicate,
    Crypt,
    IncorrectLogin,
    Forbidden,
    Chrono,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    fn new(
        message: Option<String>,
        cause: Option<Box<dyn std::error::Error>>,
        types: AppErrorType,
    ) -> Self {
        Self {
            message,
            cause,
            types,
        }
    }

    fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }

    fn from_str(message: &str, types: AppErrorType) -> Self {
        Self::new(Some(message.to_string()), None, types)
    }

    pub fn notfound_opt(message: Option<String>) -> Self {
        Self::new(message, None, AppErrorType::NotFound)
    }

    pub fn notfound_msg(msg: &str) -> Self {
        Self::notfound_opt(Some(msg.to_string()))
    }

    pub fn notfound() -> Self {
        Self::notfound_msg("没有找到符合条件的数据")
    }

    pub fn duplicate(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::Duplicate)
    }

    pub fn incorrect_login() -> Self {
        Self::from_str("错误的邮箱或密码", AppErrorType::IncorrectLogin)
    }

    pub fn forbidden() -> Self {
        Self::from_str("无权访问", AppErrorType::Forbidden)
    }

    pub fn response(self) -> axum::response::Response {
        match self.types {
            AppErrorType::Forbidden => {
                let mut hm = HeaderMap::new();
                hm.insert(header::LOCATION, "/auth".parse().unwrap());
                (StatusCode::FOUND, hm, ()).into_response()
            }
            _ => self
                .message
                .to_owned()
                .unwrap_or("有错误发生".to_string())
                .into_response(),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}
 

/**
 * 将其他类型的Error转换为AppError
 */
//postgres数据库连接池异常
impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

//postgres查询异常
impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

//模板操作异常
impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Template)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Crypt)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Chrono)
    }
}

//以作为axum的response
impl IntoResponse for AppError {
    // fn into_response(self) -> axum::response::Response {
    //     let msg = match self.message {
    //         Some(msg) => msg.clone(),
    //         None => "发生错误".to_string(),
    //     };
    //     msg.into_response()
    // }

    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}

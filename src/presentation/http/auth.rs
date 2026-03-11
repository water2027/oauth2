use axum::{
    extract::State,
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::context::identity::application::auth_app::AuthAppService;
use crate::context::identity::application::command::{LoginCommand, RegisterCommand};
use crate::context::identity::error::DomainError;
use crate::context::identity::value_object::{
    email::Email,
    raw_password::RawPassword,
    username::Username,
    validation_code::ValidationCode,
};
use super::{success, fail, ApiResult};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub validation_code: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub cookie: String,
    pub user_id: String,
}

pub async fn register(
    State(app_service): State<Arc<AuthAppService>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let cmd = match (
        Username::parse(payload.username),
        Email::parse(payload.email),
        RawPassword::new(payload.password),
        RawPassword::new(payload.password_confirm),
        ValidationCode::new(payload.validation_code),
    ) {
        (Ok(un), Ok(em), Ok(pw), Ok(pwc), Ok(vc)) => RegisterCommand {
            username: un,
            email: em,
            password: pw,
            password_confirm: pwc,
            validation_code: vc,
        },
        _ => return fail(StatusCode::BAD_REQUEST, 400, "参数格式错误"),
    };

    match app_service.register(cmd).await {
        Ok(session) => success(SessionResponse {
            cookie: session.cookie,
            user_id: session.user_id.as_ref().to_string(),
        }),
        Err(e) => to_api_error(e),
    }
}

pub async fn login(
    State(app_service): State<Arc<AuthAppService>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let cmd = match (
        Email::parse(payload.email),
        RawPassword::new(payload.password),
    ) {
        (Ok(em), Ok(pw)) => LoginCommand { email: em, password: pw },
        _ => return fail(StatusCode::BAD_REQUEST, 400, "参数格式错误"),
    };

    match app_service.login(cmd).await {
        Ok(session) => success(SessionResponse {
            cookie: session.cookie,
            user_id: session.user_id.as_ref().to_string(),
        }),
        Err(e) => to_api_error(e),
    }
}

fn to_api_error<T: Serialize>(err: DomainError) -> ApiResult<T> {
    let status = match err {
        DomainError::UserNotFound => StatusCode::NOT_FOUND,
        DomainError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        DomainError::EmailAlreadyExists => StatusCode::CONFLICT,
        DomainError::InternalError(_) | DomainError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::BAD_REQUEST,
    };

    fail(status, err.code(), &err.to_string())
}

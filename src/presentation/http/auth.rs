use axum::{
    extract::State,
    Json,
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Duration;

use crate::context::identity::application::auth_app::AuthAppService;
use crate::context::identity::application::command::{LoginCommand, RegisterCommand};
use crate::context::identity::error::DomainError;
use crate::context::identity::entity::session::Session;
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
    pub user_id: String,
}
#[derive(Deserialize)]
pub struct SendCodeRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub new_password: String,
    pub password_confirm: String,
    pub validation_code: String,
}

pub async fn send_code(
    State(app_service): State<Arc<AuthAppService>>,
    Json(payload): Json<SendCodeRequest>,
) -> impl IntoResponse {
    let email = match Email::parse(payload.email) {
        Ok(em) => em,
        Err(_) => return fail::<()>(StatusCode::BAD_REQUEST, 400, "邮箱格式错误").into_response(),
    };

    match app_service.send_validation_code(email).await {
        Ok(_) => success(()).into_response(),
        Err(e) => to_api_error::<()>(e).into_response(),
    }
}

pub async fn reset_password(
    State(app_service): State<Arc<AuthAppService>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    if payload.new_password != payload.password_confirm {
        return fail::<()>(StatusCode::BAD_REQUEST, 400, "两次输入的密码不一致").into_response();
    }

    let email = match Email::parse(payload.email) {
        Ok(em) => em,
        Err(_) => return fail::<()>(StatusCode::BAD_REQUEST, 400, "邮箱格式错误").into_response(),
    };

    let new_pass = match RawPassword::new(payload.new_password) {
        Ok(pw) => pw,
        Err(_) => return fail::<()>(StatusCode::BAD_REQUEST, 400, "密码格式错误").into_response(),
    };

    let code = match ValidationCode::new(payload.validation_code) {
        Ok(vc) => vc,
        Err(_) => return fail::<()>(StatusCode::BAD_REQUEST, 400, "验证码格式错误").into_response(),
    };

    match app_service.reset_password(email, new_pass, code).await {
        Ok(_) => success(()).into_response(),
        Err(e) => to_api_error::<()>(e).into_response(),
    }
}

pub async fn me(
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    success(SessionResponse {
        user_id: session.user_id.as_ref().to_string(),
    }).into_response()
}

pub async fn register(
    State(app_service): State<Arc<AuthAppService>>,
    jar: CookieJar,
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
        _ => return (jar, fail::<SessionResponse>(StatusCode::BAD_REQUEST, 400, "参数格式错误")).into_response(),
    };

    match app_service.register(cmd).await {
        Ok(session) => {
            let cookie = Cookie::build(("session", session.cookie))
                .path("/")
                .http_only(true)
                .max_age(Duration::seconds(7 * 24 * 3600))
                .build();
            (jar.add(cookie), success(SessionResponse {
                user_id: session.user_id.as_ref().to_string(),
            })).into_response()
        }
        Err(e) => (jar, to_api_error::<SessionResponse>(e)).into_response(),
    }
}

pub async fn login(
    State(app_service): State<Arc<AuthAppService>>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let cmd = match (
        Email::parse(payload.email),
        RawPassword::new(payload.password),
    ) {
        (Ok(em), Ok(pw)) => LoginCommand { email: em, password: pw },
        _ => return (jar, fail::<SessionResponse>(StatusCode::BAD_REQUEST, 400, "参数格式错误")).into_response(),
    };

    match app_service.login(cmd).await {
        Ok(session) => {
            let cookie = Cookie::build(("session", session.cookie))
                .path("/")
                .http_only(true)
                .max_age(Duration::seconds(7 * 24 * 3600))
                .build();
            (jar.add(cookie), success(SessionResponse {
                user_id: session.user_id.as_ref().to_string(),
            })).into_response()
        }
        Err(e) => (jar, to_api_error::<SessionResponse>(e)).into_response(),
    }
}

pub async fn logout(
    State(app_service): State<Arc<AuthAppService>>,
    Extension(session): Extension<Session>,
    jar: CookieJar,
) -> impl IntoResponse {
    // 因为中间件已经校验过 session，这里直接使用即可
    let _ = app_service.logout(&session.cookie).await;
    (jar.remove(Cookie::from("session")), success(())).into_response()
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

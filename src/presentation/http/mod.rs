use axum::{
    routing::post,
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use async_trait::async_trait;

use crate::context::identity::application::auth_app::AuthAppService;
use crate::presentation::IHttpEngine;

pub mod auth;

/// 统一响应结构
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

/// 统一结果封装
pub struct ApiResult<T>(pub StatusCode, pub ApiResponse<T>);

impl<T: Serialize> IntoResponse for ApiResult<T> {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}

pub fn success<T: Serialize>(data: T) -> ApiResult<T> {
    ApiResult(
        StatusCode::OK,
        ApiResponse {
            code: 100,
            message: "success".to_string(),
            data: Some(data),
        },
    )
}

pub fn fail<T: Serialize>(status: StatusCode, code: i32, message: &str) -> ApiResult<T> {
    ApiResult(
        status,
        ApiResponse {
            code,
            message: message.to_string(),
            data: None,
        },
    )
}

// --- Axum 引擎实现 ---

pub struct AxumHttpEngine {
    auth_app_service: Arc<AuthAppService>,
}

impl AxumHttpEngine {
    pub fn new(auth_app_service: Arc<AuthAppService>) -> Self {
        Self { auth_app_service }
    }
}

#[async_trait]
impl IHttpEngine for AxumHttpEngine {
    async fn start(&self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        // 配置路由
        let app = Router::new()
            .route("/auth/register", post(auth::register))
            .route("/auth/login", post(auth::login))
            .with_state(self.auth_app_service.clone());

        // 启动服务器
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("Axum engine started on {}", addr);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

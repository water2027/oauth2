use axum::{
    routing::{get, post},
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    middleware::from_fn_with_state,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use async_trait::async_trait;

use crate::context::identity::application::auth_app::AuthAppService;
use crate::presentation::IHttpEngine;

pub mod middleware;
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
        // 分离公开路由和受保护路由
        let public_routes = Router::new()
            .route("/auth/register", post(auth::register))
            .route("/auth/login", post(auth::login))
            .route("/auth/send-code", post(auth::send_code))
            .route("/auth/reset-password", post(auth::reset_password));

        let protected_routes = Router::new()
            .route("/auth/logout", post(auth::logout))
            .route("/auth/me", get(auth::me))
            // 可以继续添加更多受保护路由
            .route_layer(from_fn_with_state(
                self.auth_app_service.clone(),
                middleware::auth::auth_middleware,
            ));

        // 合并路由并注入状态
        let app = Router::new()
            .merge(public_routes)
            .merge(protected_routes)
            .with_state(self.auth_app_service.clone());

        // 启动服务器
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("Axum engine started on {}", addr);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

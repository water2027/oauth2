use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
    http::StatusCode,
};
use axum_extra::extract::CookieJar;
use std::sync::Arc;

use crate::context::identity::application::auth_app::AuthAppService;
use crate::presentation::http::fail;

pub async fn auth_middleware(
    State(app_service): State<Arc<AuthAppService>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    let cookie = jar.get("session");
    
    match cookie {
        Some(cookie) => {
            match app_service.verify(cookie.value()).await {
                Ok(Some(mut session)) => {
                    app_service.refresh(&mut session).await.unwrap_or(());
                    req.extensions_mut().insert(session);
                    next.run(req).await
                }
                Ok(None) => {
                    fail::<()>(StatusCode::UNAUTHORIZED, 401, "Invalid session").into_response()
                }
                Err(_) => {
                    fail::<()>(StatusCode::INTERNAL_SERVER_ERROR, 500, "Internal server error").into_response()
                }
            }
        }
        None => {
            fail::<()>(StatusCode::UNAUTHORIZED, 401, "Session not found").into_response()
        }
    }
}

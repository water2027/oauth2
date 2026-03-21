use crate::context::oauth::value_object::{AuthTicket, ClientID, RedirectUri, Scope, UserID};

/// 授权请求的响应类型
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseType {
    Token,
    Code,
}

/// PKCE 挑战码方法
#[derive(Debug, Clone, PartialEq)]
pub enum CodeChallengeMethod {
    S256,
    // 传就报错
    Plain,
}

/// 授权请求实体
///
/// 当客户端（如移动端、SPA）发起授权请求时，该实体用于在用户登录和授权过程中
/// 暂存请求的上下文状态。
pub struct AuthRequest {
    pub id: AuthTicket,
    pub client_id: ClientID,
    pub user_id: UserID,
    pub redirect_uri: RedirectUri,
    pub scope: Scope,
    pub response_type: ResponseType,
    /// 客户端用于防止 CSRF 的状态字符串
    pub state: Option<String>,
    /// PKCE 挑战码 (强制性，以符合 OAuth 2.1 标准)
    pub code_challenge: String,
    /// PKCE 挑战码方法 (通常为 S256)
    pub code_challenge_method: CodeChallengeMethod,
    /// 创建时间，用于计算过期
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuthRequest {
    pub fn new(
        id: AuthTicket,
        client_id: ClientID,
        user_id: UserID,
        redirect_uri: RedirectUri,
        scope: Scope,
        response_type: ResponseType,
        state: Option<String>,
        code_challenge: String,
        code_challenge_method: CodeChallengeMethod,
    ) -> Self {
        Self {
            id,
            client_id,
            user_id,
            redirect_uri,
            scope,
            response_type,
            state,
            code_challenge,
            code_challenge_method,
            created_at: chrono::Utc::now(),
        }
    }

    /// 检查授权请求是否已过期
    ///
    /// 授权请求是一个中间状态，有效期通常很短（5-10 分钟）。
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(self.created_at);
        duration.num_minutes() > 10
    }
}

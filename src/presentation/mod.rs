pub mod http;

use async_trait::async_trait;

#[async_trait]
pub trait IHttpEngine: Send + Sync {
    /// 启动 Web 服务器
    async fn start(&self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>>;
}

use async_trait::async_trait;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use super::EmailSender;

pub struct QQEmailSender {
    smtp_server: String,
    username: String,
    password: String,
}

impl QQEmailSender {
    pub fn new(username: String, password: String) -> Self {
        Self {
            smtp_server: "smtp.qq.com".to_string(),
            username,
            password,
        }
    }
}

#[async_trait]
impl EmailSender for QQEmailSender {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        // 1. 构建邮件消息
        let email = Message::builder()
            .from(
                format!("OAuth2 Service <{}>", self.username)
                    .parse()
                    .map_err(|e| format!("Invalid from address: {}", e))?,
            )
            .to(to
                .parse()
                .map_err(|e| format!("Invalid recipient address: {}", e))?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| format!("Failed to build email message: {}", e))?;

        // 2. 设置凭据 (QQ邮箱授权码)
        let creds = Credentials::new(self.username.clone(), self.password.clone());

        // 3. 配置 SMTP 传输
        // lettre 默认会尝试使用 STARTTLS (通常在端口 587)
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.smtp_server)
            .map_err(|e| format!("Failed to create SMTP transport: {}", e))?
            .credentials(creds)
            .build();

        // 4. 异步发送邮件
        mailer
            .send(email)
            .await
            .map_err(|e| format!("Failed to send email via SMTP: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_send_qq_email() {
        // 调用全局测试环境初始化
        crate::shared::init_test_env();

        let username = env::var("SMTP_USERNAME").unwrap_or_default();
        let password = env::var("SMTP_PASSWORD").unwrap_or_default();
        let to = env::var("SMTP_TO").unwrap_or_default();
        println!("Testing QQEmailSender from {}, to: {}", username, to);

        if username.is_empty() || password.is_empty() || to.is_empty() {
            println!("Skipping test: SMTP_USERNAME, SMTP_PASSWORD, or SMTP_TO not set in environment.");
            return;
        }

        let sender = QQEmailSender::new(username, password);
        let result = sender.send_email(
            &to,
            "Rust OAuth2 Test (Env)",
            "This email was sent using environment variables for configuration."
        ).await;

        assert!(result.is_ok(), "Failed to send email: {:?}", result.err());
    }
}

use std::env;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 基础设施与共享层
use oauth2::context::identity::infrastructure::{
    code::EmailCodeSender, code::RedisCodeRepository, code::SimpleCodeGenerator,
    password_hasher::Argon2PasswordHasher, session::SqlxSessionRepository,
    user::SqlxUserRepository, user::UUIDUserIDGenerator,
};
use oauth2::shared::infrastructure::email::qq::QQEmailSender;

// 领域服务
use oauth2::context::identity::service::{
    code::CodeService, session::SessionService, user::UserService,
};

// 应用服务
use oauth2::context::identity::application::auth_app::AuthAppService;

// 表现层 (Engine)
use oauth2::presentation::{http::AxumHttpEngine, IHttpEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "oauth2=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. 加载环境变量
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // 3. 初始化连接池
    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    let redis_client = redis::Client::open(redis_url)?;

    // 4. 构建基础设施
    let user_repo = Arc::new(SqlxUserRepository::new(db_pool.clone()));
    let session_repo = Arc::new(SqlxSessionRepository::new(db_pool.clone()));
    let code_repo = Arc::new(RedisCodeRepository::new(redis_client));

    let user_id_gen = Arc::new(UUIDUserIDGenerator);
    let code_gen = Arc::new(SimpleCodeGenerator);
    let password_hasher = Arc::new(Argon2PasswordHasher);

    let smtp_user = env::var("SMTP_USERNAME")?;
    let smtp_pass = env::var("SMTP_PASSWORD")?;
    let email_sender = Arc::new(QQEmailSender::new(smtp_user, smtp_pass));
    let code_sender = Arc::new(EmailCodeSender::new(email_sender));

    // 5. 构建领域服务
    let user_service = UserService::new(user_repo, user_id_gen, password_hasher);
    let session_service = SessionService::new(session_repo);
    let code_service = CodeService::new(code_repo, code_sender, code_gen);

    // 6. 构建应用服务
    let auth_app_service = Arc::new(AuthAppService::new(
        session_service,
        user_service,
        code_service,
    ));

    // 7. 使用 IHttpEngine 启动服务器
    let engine = AxumHttpEngine::new(auth_app_service);
    engine.start("0.0.0.0", 8080).await?;

    Ok(())
}

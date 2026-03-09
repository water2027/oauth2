use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryError {
    #[error("未找到")]
    NotFound,
    #[error("内部错误: {0}")]
    InternalError(String),
}
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeckError {
    #[error("Deck App 未运行或未启用 CLI Bridge")]
    NotRunning,

    #[error("连接失败: {0}")]
    Connection(String),

    #[error("认证失败: {0}")]
    Auth(String),

    #[error("Token 文件不存在: {path}")]
    TokenNotFound { path: String },

    #[error("请求超时")]
    Timeout,

    #[error("协议错误: {0}")]
    Protocol(String),

    #[error("服务端错误 [{code}]: {message}")]
    Server { code: String, message: String },

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl From<deckclip_protocol::codec::CodecError> for DeckError {
    fn from(e: deckclip_protocol::codec::CodecError) -> Self {
        DeckError::Protocol(e.to_string())
    }
}

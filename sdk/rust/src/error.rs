#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("S.I.R.K. protocol error: {0}")]
    Protocol(String),
    #[error("S.I.R.K. RPC error {code}: {message}")]
    Remote { code: i64, message: String },
}

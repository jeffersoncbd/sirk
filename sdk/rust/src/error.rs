#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not determine the current directory: {0}")]
    CurrentDirectory(#[source] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("S.I.R.K. protocol error: {0}")]
    Protocol(String),
    #[error("S.I.R.K. HTTP error {status}: {message}")]
    Remote { status: u16, message: String },
    #[error("S.I.R.K. host tool failed: {0}")]
    Tool(String),
    #[error(
        "Could not connect to S.I.R.K. at {endpoint}. Start S.I.R.K. with `sirk http`: {source}"
    )]
    Unavailable {
        endpoint: String,
        #[source]
        source: ureq::Error,
    },
}

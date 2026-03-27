use serde::Serialize;

/// Options for get statement data download url request
#[derive(Debug, Serialize, Clone)]
pub struct GetStatementOptions {
    file_key: String,
}

impl GetStatementOptions {
    /// Create a new `GetStatementDataDownloadUrlOptions`
    #[inline]
    pub fn new(file_key: impl Into<String>) -> Self {
        Self {
            file_key: file_key.into(),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl AgentResponse {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            status: "success".to_string(),
            file: None,
            hash: None,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }
}

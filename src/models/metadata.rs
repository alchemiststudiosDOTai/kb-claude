use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    pub last_updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl Metadata {
    pub fn new(component: String, summary: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/metadata/v1".to_string(),
            component,
            summary,
            dependencies: None,
            intent: None,
            last_updated: Utc::now(),
            owner: None,
            tags: None,
        }
    }
}

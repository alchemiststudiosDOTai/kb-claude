use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeltaManifest {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub changes: Vec<Change>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Change {
    pub file: String,
    pub summary: String,
    #[serde(rename = "type")]
    pub change_type: String,
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

impl DeltaManifest {
    #[allow(dead_code)]
    pub fn new(component: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/delta/v1".to_string(),
            component,
            changes: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_change(
        &mut self,
        file: String,
        summary: String,
        change_type: String,
        commit: Option<String>,
    ) {
        self.changes.push(Change {
            file,
            summary,
            change_type,
            date: Utc::now(),
            commit,
        });
    }
}

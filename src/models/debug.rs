use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebugHistory {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub entries: Vec<DebugEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebugEntry {
    pub id: String,
    pub error: String,
    pub solution: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    pub date: DateTime<Utc>,
}

impl DebugHistory {
    pub fn new(component: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/debug/v1".to_string(),
            component,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, error: String, solution: String, commit: Option<String>) {
        let id = format!("dbg_{:03}", self.entries.len() + 1);
        self.entries.push(DebugEntry {
            id,
            error,
            solution,
            commit,
            date: Utc::now(),
        });
    }
}

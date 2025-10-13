use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeIndex {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub files: Vec<FileIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileIndex {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<String>>,
}

impl CodeIndex {
    pub fn new(component: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/code_index/v1".to_string(),
            component,
            files: Vec::new(),
            last_updated: Some(Utc::now()),
        }
    }

    pub fn add_file(&mut self, path: String, description: Option<String>) {
        self.files.push(FileIndex {
            path,
            description,
            functions: None,
            classes: None,
        });
        self.last_updated = Some(Utc::now());
    }

    #[allow(dead_code)]
    pub fn update_file(&mut self, path: &str, description: Option<String>) {
        if let Some(file) = self.files.iter_mut().find(|f| f.path == path) {
            if let Some(desc) = description {
                file.description = Some(desc);
            }
        }
        self.last_updated = Some(Utc::now());
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cheatsheet {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub title: String,
    pub sections: Vec<CheatsheetSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheatsheetSection {
    pub heading: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
}

impl Cheatsheet {
    pub fn new(component: String, title: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/cheatsheet/v1".to_string(),
            component,
            title,
            sections: Vec::new(),
            last_updated: Some(Utc::now()),
        }
    }

    pub fn add_section(&mut self, heading: String, content: String) {
        self.sections.push(CheatsheetSection {
            heading,
            content,
            examples: None,
        });
        self.last_updated = Some(Utc::now());
    }

    #[allow(dead_code)]
    pub fn update_section(&mut self, heading: &str, content: Option<String>) {
        if let Some(section) = self.sections.iter_mut().find(|s| s.heading == heading) {
            if let Some(c) = content {
                section.content = c;
            }
        }
        self.last_updated = Some(Utc::now());
    }
}

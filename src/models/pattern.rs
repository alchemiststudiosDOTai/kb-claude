use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pattern {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub patterns: Vec<PatternEntry>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternEntry {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl Pattern {
    pub fn new(component: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/pattern/v1".to_string(),
            component,
            patterns: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    pub fn add_pattern(&mut self, name: String, description: String, example: Option<String>) {
        self.patterns.push(PatternEntry {
            name,
            description,
            example,
            tags: None,
        });
        self.last_updated = Utc::now();
    }

    pub fn update_pattern(
        &mut self,
        name: &str,
        description: Option<String>,
        example: Option<String>,
    ) -> bool {
        if let Some(pattern) = self.patterns.iter_mut().find(|p| p.name == name) {
            if let Some(desc) = description {
                pattern.description = desc;
            }
            if let Some(ex) = example {
                pattern.example = Some(ex);
            }

            self.last_updated = Utc::now();
            return true;
        }

        false
    }
}

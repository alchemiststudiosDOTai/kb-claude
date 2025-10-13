use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QA {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub component: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub q: String,
    pub a: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
}

impl QA {
    pub fn new(component: String) -> Self {
        Self {
            schema: "https://claude-kb.io/schema/qa/v1".to_string(),
            component,
            questions: Vec::new(),
        }
    }

    pub fn add_question(
        &mut self,
        q: String,
        a: String,
        context: Option<String>,
        source_commit: Option<String>,
    ) {
        self.questions.push(Question {
            q,
            a,
            context,
            source_commit,
        });
    }
}

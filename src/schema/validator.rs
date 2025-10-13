use anyhow::Result;
use serde_json::Value;
use std::path::Path;

use crate::io::read_json;

#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub file: String,
}

pub fn validate_file(path: &Path) -> Result<ValidationResult> {
    let file_name = path.to_string_lossy().to_string();
    let mut errors = Vec::new();

    let value: Value = match read_json(path) {
        Ok(v) => v,
        Err(e) => {
            return Ok(ValidationResult {
                valid: false,
                errors: vec![format!("Failed to parse JSON: {}", e)],
                file: file_name,
            });
        }
    };

    if let Some(obj) = value.as_object() {
        if let Some(schema) = obj.get("$schema").and_then(|s| s.as_str()) {
            if schema.contains("/metadata/") {
                validate_metadata(&value, &mut errors);
            } else if schema.contains("/debug/") {
                validate_debug(&value, &mut errors);
            } else if schema.contains("/qa/") {
                validate_qa(&value, &mut errors);
            } else if schema.contains("/delta/") {
                validate_delta(&value, &mut errors);
            } else if schema.contains("/code_index/") {
                validate_code_index(&value, &mut errors);
            } else if schema.contains("/pattern/") {
                validate_pattern(&value, &mut errors);
            } else if schema.contains("/cheatsheet/") {
                validate_cheatsheet(&value, &mut errors);
            }
        } else if obj.contains_key("component") {
            if obj.contains_key("entries") {
                validate_debug(&value, &mut errors);
            } else if obj.contains_key("questions") {
                validate_qa(&value, &mut errors);
            } else if obj.contains_key("summary") {
                validate_metadata(&value, &mut errors);
            } else if obj.contains_key("patterns") {
                validate_pattern(&value, &mut errors);
            } else if obj.contains_key("files") {
                validate_code_index(&value, &mut errors);
            }
        }
    } else {
        errors.push("Root element must be a JSON object".to_string());
    }

    Ok(ValidationResult {
        valid: errors.is_empty(),
        errors,
        file: file_name,
    })
}

fn validate_metadata(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
}

fn validate_debug(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
    if !obj.contains_key("entries") {
        errors.push("Missing required field: entries".to_string());
    } else if let Some(entries) = obj.get("entries").and_then(|e| e.as_array()) {
        for (i, entry) in entries.iter().enumerate() {
            if let Some(e) = entry.as_object() {
                let required = vec!["id", "error", "solution", "date"];
                for field in required {
                    if !e.contains_key(field) {
                        errors.push(format!("Entry {}: missing required field: {}", i, field));
                    }
                }
            }
        }
    }
}

fn validate_qa(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
    if !obj.contains_key("questions") {
        errors.push("Missing required field: questions".to_string());
    } else if let Some(questions) = obj.get("questions").and_then(|q| q.as_array()) {
        for (i, question) in questions.iter().enumerate() {
            if let Some(q) = question.as_object() {
                if !q.contains_key("q") {
                    errors.push(format!("Question {}: missing required field: q", i));
                }
                if !q.contains_key("a") {
                    errors.push(format!("Question {}: missing required field: a", i));
                }
            }
        }
    }
}

fn validate_delta(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
    if !obj.contains_key("changes") {
        errors.push("Missing required field: changes".to_string());
    } else if let Some(changes) = obj.get("changes").and_then(|c| c.as_array()) {
        for (i, change) in changes.iter().enumerate() {
            if let Some(c) = change.as_object() {
                let required = vec!["file", "summary", "type", "date"];
                for field in required {
                    if !c.contains_key(field) {
                        errors.push(format!("Change {}: missing required field: {}", i, field));
                    }
                }
            }
        }
    }
}

fn validate_code_index(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
}

fn validate_pattern(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
}

fn validate_cheatsheet(value: &Value, errors: &mut Vec<String>) {
    let obj = value.as_object().unwrap();

    if !obj.contains_key("component") {
        errors.push("Missing required field: component".to_string());
    }
}

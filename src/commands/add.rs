use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::agent::AgentResponse;
use crate::io::{ensure_claude_dirs, get_entry_path, read_json, write_json};
use crate::manifest::compute_file_hash;
use crate::models::{Cheatsheet, CodeIndex, DebugHistory, EntryType, Metadata, Pattern, QA};

#[allow(clippy::too_many_arguments)]
pub fn handle(
    entry_type: EntryType,
    component: String,
    summary: Option<String>,
    error: Option<String>,
    solution: Option<String>,
    question: Option<String>,
    answer: Option<String>,
    name: Option<String>,
    description: Option<String>,
    heading: Option<String>,
    content: Option<String>,
    file_path: Option<String>,
    note: Option<String>,
    json: bool,
) -> Result<()> {
    ensure_claude_dirs()?;

    let path = get_entry_path(&entry_type, &component);

    match entry_type {
        EntryType::Metadata => {
            let summary = summary.ok_or_else(|| anyhow!("--summary is required for metadata"))?;
            let metadata = Metadata::new(component.clone(), summary);
            write_json(&path, &metadata, true)?;
        }
        EntryType::Debug => {
            let error = error.ok_or_else(|| anyhow!("--error is required for debug"))?;
            let solution = solution.ok_or_else(|| anyhow!("--solution is required for debug"))?;

            let mut debug = if path.exists() {
                read_json(&path)?
            } else {
                DebugHistory::new(component.clone())
            };

            debug.add_entry(error, solution, None);
            write_json(&path, &debug, true)?;
        }
        EntryType::QA => {
            let question = question.ok_or_else(|| anyhow!("--question is required for qa"))?;
            let answer = answer.ok_or_else(|| anyhow!("--answer is required for qa"))?;

            let mut qa = if path.exists() {
                read_json(&path)?
            } else {
                QA::new(component.clone())
            };

            qa.add_question(question, answer, None, None);
            write_json(&path, &qa, true)?;
        }
        EntryType::CodeIndex => {
            let file_path =
                file_path.ok_or_else(|| anyhow!("--file-path is required for code_index"))?;

            let mut code_index = if path.exists() {
                read_json(&path)?
            } else {
                CodeIndex::new(component.clone())
            };

            code_index.add_file(file_path, note);
            write_json(&path, &code_index, true)?;
        }
        EntryType::Pattern => {
            let name = name.ok_or_else(|| anyhow!("--name is required for pattern"))?;
            let description =
                description.ok_or_else(|| anyhow!("--description is required for pattern"))?;

            let mut pattern = if path.exists() {
                read_json(&path)?
            } else {
                Pattern::new(component.clone())
            };

            pattern.add_pattern(name, description, None);
            write_json(&path, &pattern, true)?;
        }
        EntryType::Cheatsheet => {
            let summary =
                summary.ok_or_else(|| anyhow!("--summary (title) is required for cheatsheet"))?;

            let mut cheatsheet = if path.exists() {
                read_json(&path)?
            } else {
                Cheatsheet::new(component.clone(), summary.clone())
            };

            if let (Some(h), Some(c)) = (heading, content) {
                cheatsheet.add_section(h, c);
            }
            write_json(&path, &cheatsheet, true)?;
        }
        EntryType::Delta => {
            return Err(anyhow!(
                "Delta entries are not supported via the add command"
            ));
        }
    }

    let hash = compute_file_hash(&path)?;
    let short_hash = &hash[..8];

    if json {
        let response = AgentResponse::success(format!("{} entry created.", entry_type))
            .with_file(path.to_string_lossy().to_string())
            .with_hash(short_hash.to_string());
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        println!(
            "{} {}",
            "✓".green().bold(),
            format!("{} entry created", entry_type).bold()
        );
        println!("  File: {}", path.display());
        println!("  Hash: {}", short_hash);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::TempDir;

    fn setup_test_env() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        temp_dir
    }

    #[test]
    #[serial]
    fn test_add_metadata_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Metadata,
            "test.component".to_string(),
            Some("Test summary".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::Metadata, "test.component");
        assert!(path.exists());

        let metadata: Metadata = read_json(&path).unwrap();
        assert_eq!(metadata.component, "test.component");
        assert_eq!(metadata.summary, "Test summary");
    }

    #[test]
    #[serial]
    fn test_add_metadata_missing_summary() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Metadata,
            "test.component".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("--summary is required"));
    }

    #[test]
    #[serial]
    fn test_add_debug_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Debug,
            "test.component".to_string(),
            None,
            Some("Test error".to_string()),
            Some("Test solution".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::Debug, "test.component");
        assert!(path.exists());

        let debug: DebugHistory = read_json(&path).unwrap();
        assert_eq!(debug.component, "test.component");
        assert_eq!(debug.entries.len(), 1);
        assert_eq!(debug.entries[0].error, "Test error");
        assert_eq!(debug.entries[0].solution, "Test solution");
    }

    #[test]
    #[serial]
    fn test_add_debug_multiple_entries() {
        let _temp = setup_test_env();

        handle(
            EntryType::Debug,
            "test.component".to_string(),
            None,
            Some("Error 1".to_string()),
            Some("Solution 1".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        handle(
            EntryType::Debug,
            "test.component".to_string(),
            None,
            Some("Error 2".to_string()),
            Some("Solution 2".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        let path = get_entry_path(&EntryType::Debug, "test.component");
        let debug: DebugHistory = read_json(&path).unwrap();
        assert_eq!(debug.entries.len(), 2);
    }

    #[test]
    #[serial]
    fn test_add_qa_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::QA,
            "test.component".to_string(),
            None,
            None,
            None,
            Some("What is this?".to_string()),
            Some("This is a test".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::QA, "test.component");
        let qa: QA = read_json(&path).unwrap();
        assert_eq!(qa.questions.len(), 1);
        assert_eq!(qa.questions[0].q, "What is this?");
        assert_eq!(qa.questions[0].a, "This is a test");
    }

    #[test]
    #[serial]
    fn test_add_pattern_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Pattern,
            "test.component".to_string(),
            None,
            None,
            None,
            None,
            None,
            Some("TestPattern".to_string()),
            Some("Pattern description".to_string()),
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::Pattern, "test.component");
        let pattern: Pattern = read_json(&path).unwrap();
        assert_eq!(pattern.patterns.len(), 1);
        assert_eq!(pattern.patterns[0].name, "TestPattern");
        assert_eq!(pattern.patterns[0].description, "Pattern description");
    }

    #[test]
    #[serial]
    fn test_add_code_index_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::CodeIndex,
            "test.component".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("src/main.rs".to_string()),
            Some("Main entry point".to_string()),
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::CodeIndex, "test.component");
        let code_index: CodeIndex = read_json(&path).unwrap();
        assert_eq!(code_index.files.len(), 1);
        assert_eq!(code_index.files[0].path, "src/main.rs");
    }

    #[test]
    #[serial]
    fn test_add_cheatsheet_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Cheatsheet,
            "test.component".to_string(),
            Some("Test Cheatsheet".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("Introduction".to_string()),
            Some("This is the intro".to_string()),
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::Cheatsheet, "test.component");
        let cheatsheet: Cheatsheet = read_json(&path).unwrap();
        assert_eq!(cheatsheet.title, "Test Cheatsheet");
        assert_eq!(cheatsheet.sections.len(), 1);
        assert_eq!(cheatsheet.sections[0].heading, "Introduction");
    }

    #[test]
    #[serial]
    fn test_add_delta_entry_returns_error() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Delta,
            "test.component".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Delta entries are not supported"));
    }

    #[test]
    #[serial]
    fn test_add_creates_claude_dirs() {
        let temp = setup_test_env();
        let base_path = temp.path().join(".claude");

        assert!(!base_path.exists());

        handle(
            EntryType::Metadata,
            "test.component".to_string(),
            Some("Test".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        assert!(base_path.exists());
        assert!(base_path.join("metadata").exists());
    }

    #[test]
    #[serial]
    fn test_add_with_json_output() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Metadata,
            "test.component".to_string(),
            Some("Test summary".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        );

        assert!(result.is_ok());
    }
}

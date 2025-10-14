use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::agent::AgentResponse;
use crate::io::{get_entry_path, read_json, write_json};
use crate::manifest::compute_file_hash;
use crate::models::{Cheatsheet, CodeIndex, DebugHistory, EntryType, Metadata, Pattern, QA};

#[allow(clippy::too_many_arguments)]
pub fn handle(
    entry_type: EntryType,
    file: Option<String>,
    component: Option<String>,
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
    // Check for unsupported entry types early
    if entry_type == EntryType::Delta {
        return Err(anyhow!(
            "Delta entries are not supported via the update command"
        ));
    }

    let path = if let Some(file_path) = file {
        std::path::PathBuf::from(file_path)
    } else if let Some(comp) = component {
        get_entry_path(&entry_type, &comp)
    } else {
        return Err(anyhow!("Either --file or --component must be specified"));
    };

    if !path.exists() {
        return Err(anyhow!("File does not exist: {}", path.display()));
    }

    match entry_type {
        EntryType::Metadata => {
            let mut metadata: Metadata = read_json(&path)?;
            if let Some(s) = solution {
                metadata.summary = s;
            }
            metadata.last_updated = chrono::Utc::now();
            write_json(&path, &metadata, true)?;
        }
        EntryType::Debug => {
            let error = error.ok_or_else(|| anyhow!("--error is required for debug update"))?;
            let solution =
                solution.ok_or_else(|| anyhow!("--solution is required for debug update"))?;

            let mut debug: DebugHistory = read_json(&path)?;
            debug.add_entry(error, solution, None);
            write_json(&path, &debug, true)?;
        }
        EntryType::QA => {
            let question =
                question.ok_or_else(|| anyhow!("--question is required for qa update"))?;
            let answer = answer.ok_or_else(|| anyhow!("--answer is required for qa update"))?;

            let mut qa: QA = read_json(&path)?;
            qa.add_question(question, answer, None, None);
            write_json(&path, &qa, true)?;
        }
        EntryType::CodeIndex => {
            let file_path = file_path
                .ok_or_else(|| anyhow!("--file-path is required for code_index update"))?;

            let mut code_index: CodeIndex = read_json(&path)?;
            code_index.add_file(file_path, note);
            write_json(&path, &code_index, true)?;
        }
        EntryType::Pattern => {
            let name = name.ok_or_else(|| anyhow!("--name is required for pattern update"))?;
            let description = description
                .ok_or_else(|| anyhow!("--description is required for pattern update"))?;

            let mut pattern: Pattern = read_json(&path)?;
            if !pattern.update_pattern(&name, Some(description), None) {
                return Err(anyhow!(
                    "Pattern '{}' not found in {}. Use `claude-kb add pattern` to create it first.",
                    name,
                    path.display()
                ));
            }
            write_json(&path, &pattern, true)?;
        }
        EntryType::Cheatsheet => {
            let heading =
                heading.ok_or_else(|| anyhow!("--heading is required for cheatsheet update"))?;
            let content =
                content.ok_or_else(|| anyhow!("--content is required for cheatsheet update"))?;

            let mut cheatsheet: Cheatsheet = read_json(&path)?;
            cheatsheet.add_section(heading, content);
            write_json(&path, &cheatsheet, true)?;
        }
        EntryType::Delta => {
            unreachable!("Delta check should have been handled earlier")
        }
    }

    let hash = compute_file_hash(&path)?;
    let short_hash = &hash[..8];

    if json {
        let response = AgentResponse::success(format!("{} entry updated.", entry_type))
            .with_file(path.to_string_lossy().to_string())
            .with_hash(short_hash.to_string());
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        println!(
            "{} {}",
            "✓".green().bold(),
            format!("{} entry updated", entry_type).bold()
        );
        println!("  File: {}", path.display());
        println!("  Hash: {}", short_hash);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::add;
    use serial_test::serial;
    use tempfile::TempDir;

    fn setup_test_env() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        temp_dir
    }

    fn create_test_metadata(component: &str) {
        add::handle(
            EntryType::Metadata,
            component.to_string(),
            Some("Initial summary".to_string()),
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
    }

    fn create_test_pattern(component: &str) {
        add::handle(
            EntryType::Pattern,
            component.to_string(),
            None,
            None,
            None,
            None,
            None,
            Some("TestPattern".to_string()),
            Some("Initial description".to_string()),
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();
    }

    #[test]
    #[serial]
    fn test_update_metadata_entry() {
        let _temp = setup_test_env();
        create_test_metadata("test.component");

        let result = handle(
            EntryType::Metadata,
            None,
            Some("test.component".to_string()),
            None,
            Some("Updated summary".to_string()),
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
        let metadata: Metadata = read_json(&path).unwrap();
        assert_eq!(metadata.summary, "Updated summary");
    }

    #[test]
    #[serial]
    fn test_update_nonexistent_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Metadata,
            None,
            Some("nonexistent.component".to_string()),
            None,
            Some("Updated summary".to_string()),
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
            .contains("File does not exist"));
    }

    #[test]
    #[serial]
    fn test_update_debug_adds_entry() {
        let _temp = setup_test_env();

        add::handle(
            EntryType::Debug,
            "test.component".to_string(),
            None,
            Some("Initial error".to_string()),
            Some("Initial solution".to_string()),
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

        let result = handle(
            EntryType::Debug,
            None,
            Some("test.component".to_string()),
            Some("New error".to_string()),
            Some("New solution".to_string()),
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
        let debug: DebugHistory = read_json(&path).unwrap();
        assert_eq!(debug.entries.len(), 2);
        assert_eq!(debug.entries[1].error, "New error");
    }

    #[test]
    #[serial]
    fn test_update_pattern_existing() {
        let _temp = setup_test_env();
        create_test_pattern("test.component");

        let result = handle(
            EntryType::Pattern,
            None,
            Some("test.component".to_string()),
            None,
            None,
            None,
            None,
            Some("TestPattern".to_string()),
            Some("Updated description".to_string()),
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::Pattern, "test.component");
        let pattern: Pattern = read_json(&path).unwrap();
        assert_eq!(pattern.patterns[0].description, "Updated description");
    }

    #[test]
    #[serial]
    fn test_update_pattern_nonexistent_pattern() {
        let _temp = setup_test_env();
        create_test_pattern("test.component");

        let result = handle(
            EntryType::Pattern,
            None,
            Some("test.component".to_string()),
            None,
            None,
            None,
            None,
            Some("NonexistentPattern".to_string()),
            Some("Description".to_string()),
            None,
            None,
            None,
            None,
            false,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    #[serial]
    fn test_update_qa_adds_question() {
        let _temp = setup_test_env();

        add::handle(
            EntryType::QA,
            "test.component".to_string(),
            None,
            None,
            None,
            Some("Question 1".to_string()),
            Some("Answer 1".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        let result = handle(
            EntryType::QA,
            None,
            Some("test.component".to_string()),
            None,
            None,
            Some("Question 2".to_string()),
            Some("Answer 2".to_string()),
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
        assert_eq!(qa.questions.len(), 2);
    }

    #[test]
    #[serial]
    fn test_update_code_index_adds_file() {
        let _temp = setup_test_env();

        add::handle(
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
            None,
            false,
        )
        .unwrap();

        let result = handle(
            EntryType::CodeIndex,
            None,
            Some("test.component".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("src/lib.rs".to_string()),
            Some("Library code".to_string()),
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::CodeIndex, "test.component");
        let code_index: CodeIndex = read_json(&path).unwrap();
        assert_eq!(code_index.files.len(), 2);
    }

    #[test]
    #[serial]
    fn test_update_cheatsheet_adds_section() {
        let _temp = setup_test_env();

        add::handle(
            EntryType::Cheatsheet,
            "test.component".to_string(),
            Some("Test Cheatsheet".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("Section 1".to_string()),
            Some("Content 1".to_string()),
            None,
            None,
            false,
        )
        .unwrap();

        let result = handle(
            EntryType::Cheatsheet,
            None,
            Some("test.component".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("Section 2".to_string()),
            Some("Content 2".to_string()),
            None,
            None,
            false,
        );

        assert!(result.is_ok());

        let path = get_entry_path(&EntryType::Cheatsheet, "test.component");
        let cheatsheet: Cheatsheet = read_json(&path).unwrap();
        assert_eq!(cheatsheet.sections.len(), 2);
    }

    #[test]
    #[serial]
    fn test_update_with_file_path() {
        let _temp = setup_test_env();
        create_test_metadata("test.component");

        let path = get_entry_path(&EntryType::Metadata, "test.component");

        let result = handle(
            EntryType::Metadata,
            Some(path.to_string_lossy().to_string()),
            None,
            None,
            Some("Updated via file path".to_string()),
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

        let metadata: Metadata = read_json(&path).unwrap();
        assert_eq!(metadata.summary, "Updated via file path");
    }

    #[test]
    #[serial]
    fn test_update_missing_component_and_file() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Metadata,
            None,
            None,
            None,
            Some("Updated".to_string()),
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
            .contains("Either --file or --component must be specified"));
    }

    #[test]
    #[serial]
    fn test_update_delta_returns_error() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Delta,
            None,
            Some("test.component".to_string()),
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
}

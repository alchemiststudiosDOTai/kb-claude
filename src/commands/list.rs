use anyhow::Result;
use colored::Colorize;
use serde_json::Value;

use crate::io::{list_entries, read_json};
use crate::models::EntryType;

pub fn handle(
    entry_type: Option<EntryType>,
    component: Option<String>,
    json_output: bool,
) -> Result<()> {
    let paths = list_entries(entry_type.as_ref())?;

    let mut entries = Vec::new();

    for path in paths {
        let value: Value = read_json(&path)?;

        if let Some(comp_filter) = &component {
            if let Some(comp) = value.get("component").and_then(|c| c.as_str()) {
                if comp != comp_filter {
                    continue;
                }
            } else {
                continue;
            }
        }

        entries.push((path, value));
    }

    if json_output {
        let list: Vec<Value> = entries.iter().map(|(_, v)| v.clone()).collect();
        println!("{}", serde_json::to_string_pretty(&list)?);
    } else {
        if entries.is_empty() {
            println!("{}", "No entries found".yellow());
            return Ok(());
        }

        println!("{}", "Knowledge Base Entries".bold().underline());
        println!();

        for (path, value) in &entries {
            let component = value
                .get("component")
                .and_then(|c| c.as_str())
                .unwrap_or("unknown");

            let type_name = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            println!("{} {}", "•".cyan().bold(), component.bold());
            println!("  Type: {}", type_name);
            println!("  File: {}", path.display());

            if let Some(summary) = value.get("summary").and_then(|s| s.as_str()) {
                println!("  Summary: {}", summary);
            }

            println!();
        }

        println!("Total: {} entries", entries.len());
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

    fn create_multiple_entries() {
        add::handle(
            EntryType::Metadata,
            "component1".to_string(),
            Some("Summary 1".to_string()),
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

        add::handle(
            EntryType::Metadata,
            "component2".to_string(),
            Some("Summary 2".to_string()),
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

        add::handle(
            EntryType::Debug,
            "component1".to_string(),
            None,
            Some("Error".to_string()),
            Some("Solution".to_string()),
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

    #[test]
    #[serial]
    fn test_list_all_entries() {
        let _temp = setup_test_env();
        create_multiple_entries();

        let result = handle(None, None, false);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_by_entry_type() {
        let _temp = setup_test_env();
        create_multiple_entries();

        let result = handle(Some(EntryType::Metadata), None, false);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_by_component() {
        let _temp = setup_test_env();
        create_multiple_entries();

        let result = handle(None, Some("component1".to_string()), false);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_by_type_and_component() {
        let _temp = setup_test_env();
        create_multiple_entries();

        let result = handle(
            Some(EntryType::Metadata),
            Some("component1".to_string()),
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_empty_kb() {
        let _temp = setup_test_env();

        let result = handle(None, None, false);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_with_json_output() {
        let _temp = setup_test_env();
        create_multiple_entries();

        let result = handle(None, None, true);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_nonexistent_component() {
        let _temp = setup_test_env();
        create_multiple_entries();

        let result = handle(None, Some("nonexistent".to_string()), false);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_specific_entry_types() {
        let _temp = setup_test_env();

        add::handle(
            EntryType::Pattern,
            "patterns.test".to_string(),
            None,
            None,
            None,
            None,
            None,
            Some("Pattern1".to_string()),
            Some("Description".to_string()),
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        add::handle(
            EntryType::QA,
            "qa.test".to_string(),
            None,
            None,
            None,
            Some("Question".to_string()),
            Some("Answer".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        add::handle(
            EntryType::CodeIndex,
            "code.test".to_string(),
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

        assert!(handle(Some(EntryType::Pattern), None, false).is_ok());
        assert!(handle(Some(EntryType::QA), None, false).is_ok());
        assert!(handle(Some(EntryType::CodeIndex), None, false).is_ok());
    }
}

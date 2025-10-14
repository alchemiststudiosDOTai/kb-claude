use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;

use crate::agent::AgentResponse;
use crate::io::get_entry_path;
use crate::models::EntryType;

pub fn handle(
    entry_type: EntryType,
    component: Option<String>,
    file: Option<String>,
    json: bool,
) -> Result<()> {
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

    let file_display = path.display().to_string();
    fs::remove_file(&path)?;

    if json {
        let response = AgentResponse::success(format!("{} entry deleted.", entry_type))
            .with_file(file_display);
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        println!(
            "{} {}",
            "✓".green().bold(),
            format!("{} entry deleted", entry_type).bold()
        );
        println!("  File: {}", file_display);
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
        )
        .unwrap();
    }

    #[test]
    #[serial]
    fn test_delete_existing_entry() {
        let _temp = setup_test_env();
        create_test_metadata("test.component");

        let path = get_entry_path(&EntryType::Metadata, "test.component");
        assert!(path.exists());

        let result = handle(
            EntryType::Metadata,
            Some("test.component".to_string()),
            None,
            false,
        );

        assert!(result.is_ok());
        assert!(!path.exists());
    }

    #[test]
    #[serial]
    fn test_delete_nonexistent_entry() {
        let _temp = setup_test_env();

        let result = handle(
            EntryType::Metadata,
            Some("nonexistent.component".to_string()),
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
    fn test_delete_with_file_path() {
        let _temp = setup_test_env();
        create_test_metadata("test.component");

        let path = get_entry_path(&EntryType::Metadata, "test.component");
        assert!(path.exists());

        let result = handle(
            EntryType::Metadata,
            None,
            Some(path.to_string_lossy().to_string()),
            false,
        );

        assert!(result.is_ok());
        assert!(!path.exists());
    }

    #[test]
    #[serial]
    fn test_delete_missing_component_and_file() {
        let _temp = setup_test_env();

        let result = handle(EntryType::Metadata, None, None, false);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Either --file or --component must be specified"));
    }

    #[test]
    #[serial]
    fn test_delete_all_entry_types() {
        let _temp = setup_test_env();

        add::handle(
            EntryType::Debug,
            "test.debug".to_string(),
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

        add::handle(
            EntryType::QA,
            "test.qa".to_string(),
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
            EntryType::Pattern,
            "test.pattern".to_string(),
            None,
            None,
            None,
            None,
            None,
            Some("Pattern".to_string()),
            Some("Description".to_string()),
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

        let debug_path = get_entry_path(&EntryType::Debug, "test.debug");
        let qa_path = get_entry_path(&EntryType::QA, "test.qa");
        let pattern_path = get_entry_path(&EntryType::Pattern, "test.pattern");

        assert!(debug_path.exists());
        assert!(qa_path.exists());
        assert!(pattern_path.exists());

        assert!(handle(
            EntryType::Debug,
            Some("test.debug".to_string()),
            None,
            false
        )
        .is_ok());
        assert!(handle(EntryType::QA, Some("test.qa".to_string()), None, false).is_ok());
        assert!(handle(
            EntryType::Pattern,
            Some("test.pattern".to_string()),
            None,
            false
        )
        .is_ok());

        assert!(!debug_path.exists());
        assert!(!qa_path.exists());
        assert!(!pattern_path.exists());
    }

    #[test]
    #[serial]
    fn test_delete_with_json_output() {
        let _temp = setup_test_env();
        create_test_metadata("test.component");

        let result = handle(
            EntryType::Metadata,
            Some("test.component".to_string()),
            None,
            true,
        );

        assert!(result.is_ok());
    }
}

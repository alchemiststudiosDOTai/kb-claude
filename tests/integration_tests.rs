use anyhow::Result;
use serial_test::serial;
use tempfile::TempDir;

use claude_kb_cli::commands::{add, delete, list, update};
use claude_kb_cli::io::{get_entry_path, read_json};
use claude_kb_cli::models::{
    Cheatsheet, CodeIndex, DebugHistory, EntryType, Metadata, Pattern, QA,
};

fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    temp_dir
}

#[test]
#[serial]
fn test_full_crud_workflow_metadata() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE
    add::handle(
        EntryType::Metadata,
        "auth.service".to_string(),
        Some("Authentication service component".to_string()),
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
    )?;

    let path = get_entry_path(&EntryType::Metadata, "auth.service");
    assert!(path.exists());

    // READ
    let metadata: Metadata = read_json(&path)?;
    assert_eq!(metadata.component, "auth.service");
    assert_eq!(metadata.summary, "Authentication service component");

    // UPDATE
    update::handle(
        EntryType::Metadata,
        None,
        Some("auth.service".to_string()),
        None,
        Some("Updated authentication service".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    let updated_metadata: Metadata = read_json(&path)?;
    assert_eq!(updated_metadata.summary, "Updated authentication service");

    // DELETE
    delete::handle(
        EntryType::Metadata,
        Some("auth.service".to_string()),
        None,
        false,
    )?;

    assert!(!path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_full_crud_workflow_pattern() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE - add first pattern
    add::handle(
        EntryType::Pattern,
        "ui.forms".to_string(),
        None,
        None,
        None,
        None,
        None,
        Some("ValidationPattern".to_string()),
        Some("Form validation pattern".to_string()),
        None,
        None,
        None,
        None,
        false,
    )?;

    let path = get_entry_path(&EntryType::Pattern, "ui.forms");
    assert!(path.exists());

    // READ
    let pattern: Pattern = read_json(&path)?;
    assert_eq!(pattern.patterns.len(), 1);
    assert_eq!(pattern.patterns[0].name, "ValidationPattern");

    // UPDATE - add another pattern
    add::handle(
        EntryType::Pattern,
        "ui.forms".to_string(),
        None,
        None,
        None,
        None,
        None,
        Some("SubmitPattern".to_string()),
        Some("Form submission pattern".to_string()),
        None,
        None,
        None,
        None,
        false,
    )?;

    let updated_pattern: Pattern = read_json(&path)?;
    assert_eq!(updated_pattern.patterns.len(), 2);

    // UPDATE - modify existing pattern
    update::handle(
        EntryType::Pattern,
        None,
        Some("ui.forms".to_string()),
        None,
        None,
        None,
        None,
        Some("ValidationPattern".to_string()),
        Some("Updated validation pattern".to_string()),
        None,
        None,
        None,
        None,
        false,
    )?;

    let modified_pattern: Pattern = read_json(&path)?;
    assert_eq!(
        modified_pattern.patterns[0].description,
        "Updated validation pattern"
    );

    // DELETE
    delete::handle(
        EntryType::Pattern,
        Some("ui.forms".to_string()),
        None,
        false,
    )?;
    assert!(!path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_full_crud_workflow_debug_history() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE
    add::handle(
        EntryType::Debug,
        "api.users".to_string(),
        None,
        Some("500 error on user creation".to_string()),
        Some("Fixed database constraint".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    let path = get_entry_path(&EntryType::Debug, "api.users");
    assert!(path.exists());

    // READ
    let debug: DebugHistory = read_json(&path)?;
    assert_eq!(debug.entries.len(), 1);
    assert_eq!(debug.entries[0].error, "500 error on user creation");

    // UPDATE - add more debug entries
    update::handle(
        EntryType::Debug,
        None,
        Some("api.users".to_string()),
        Some("Timeout on user lookup".to_string()),
        Some("Added index on email column".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    let updated_debug: DebugHistory = read_json(&path)?;
    assert_eq!(updated_debug.entries.len(), 2);

    // DELETE
    delete::handle(EntryType::Debug, Some("api.users".to_string()), None, false)?;
    assert!(!path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_crud_workflow_with_multiple_components() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE multiple components
    let components = vec![
        ("auth.login", "Login component"),
        ("auth.signup", "Signup component"),
        ("auth.reset", "Password reset component"),
    ];

    for (component, summary) in &components {
        add::handle(
            EntryType::Metadata,
            component.to_string(),
            Some(summary.to_string()),
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
        )?;
    }

    // LIST - verify all created
    list::handle(Some(EntryType::Metadata), None, false)?;

    // UPDATE one component
    update::handle(
        EntryType::Metadata,
        None,
        Some("auth.login".to_string()),
        None,
        Some("Updated login component".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    let path = get_entry_path(&EntryType::Metadata, "auth.login");
    let metadata: Metadata = read_json(&path)?;
    assert_eq!(metadata.summary, "Updated login component");

    // DELETE all components
    for (component, _) in &components {
        delete::handle(
            EntryType::Metadata,
            Some(component.to_string()),
            None,
            false,
        )?;
    }

    // Verify all deleted
    for (component, _) in &components {
        let path = get_entry_path(&EntryType::Metadata, component);
        assert!(!path.exists());
    }

    Ok(())
}

#[test]
#[serial]
fn test_qa_workflow() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE
    add::handle(
        EntryType::QA,
        "database.setup".to_string(),
        None,
        None,
        None,
        Some("How do we handle migrations?".to_string()),
        Some("We use diesel migrations".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    // READ
    let path = get_entry_path(&EntryType::QA, "database.setup");
    let qa: QA = read_json(&path)?;
    assert_eq!(qa.questions.len(), 1);
    assert_eq!(qa.questions[0].q, "How do we handle migrations?");

    // UPDATE - add more questions
    update::handle(
        EntryType::QA,
        None,
        Some("database.setup".to_string()),
        None,
        None,
        Some("What database do we use?".to_string()),
        Some("PostgreSQL 14".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    let updated_qa: QA = read_json(&path)?;
    assert_eq!(updated_qa.questions.len(), 2);

    // DELETE
    delete::handle(
        EntryType::QA,
        Some("database.setup".to_string()),
        None,
        false,
    )?;
    assert!(!path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_code_index_workflow() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE
    add::handle(
        EntryType::CodeIndex,
        "api.handlers".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("src/handlers/users.rs".to_string()),
        Some("User management handlers".to_string()),
        false,
    )?;

    // READ
    let path = get_entry_path(&EntryType::CodeIndex, "api.handlers");
    let code_index: CodeIndex = read_json(&path)?;
    assert_eq!(code_index.files.len(), 1);
    assert_eq!(code_index.files[0].path, "src/handlers/users.rs");

    // UPDATE - add more files
    update::handle(
        EntryType::CodeIndex,
        None,
        Some("api.handlers".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("src/handlers/auth.rs".to_string()),
        Some("Authentication handlers".to_string()),
        false,
    )?;

    let updated_code_index: CodeIndex = read_json(&path)?;
    assert_eq!(updated_code_index.files.len(), 2);

    // DELETE
    delete::handle(
        EntryType::CodeIndex,
        Some("api.handlers".to_string()),
        None,
        false,
    )?;
    assert!(!path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_cheatsheet_workflow() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE
    add::handle(
        EntryType::Cheatsheet,
        "rust.basics".to_string(),
        Some("Rust Quick Reference".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        Some("Variables".to_string()),
        Some("let x = 5; // immutable\nlet mut y = 10; // mutable".to_string()),
        None,
        None,
        false,
    )?;

    // READ
    let path = get_entry_path(&EntryType::Cheatsheet, "rust.basics");
    let cheatsheet: Cheatsheet = read_json(&path)?;
    assert_eq!(cheatsheet.title, "Rust Quick Reference");
    assert_eq!(cheatsheet.sections.len(), 1);

    // UPDATE - add more sections
    update::handle(
        EntryType::Cheatsheet,
        None,
        Some("rust.basics".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        Some("Functions".to_string()),
        Some("fn add(a: i32, b: i32) -> i32 { a + b }".to_string()),
        None,
        None,
        false,
    )?;

    let updated_cheatsheet: Cheatsheet = read_json(&path)?;
    assert_eq!(updated_cheatsheet.sections.len(), 2);

    // DELETE
    delete::handle(
        EntryType::Cheatsheet,
        Some("rust.basics".to_string()),
        None,
        false,
    )?;
    assert!(!path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_mixed_entry_types_workflow() -> Result<()> {
    let _temp = setup_test_env();

    // CREATE multiple entry types for the same component
    add::handle(
        EntryType::Metadata,
        "payment.gateway".to_string(),
        Some("Payment gateway integration".to_string()),
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
    )?;

    add::handle(
        EntryType::Debug,
        "payment.gateway".to_string(),
        None,
        Some("Payment timeout error".to_string()),
        Some("Increased timeout to 30s".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    add::handle(
        EntryType::QA,
        "payment.gateway".to_string(),
        None,
        None,
        None,
        Some("Which payment providers are supported?".to_string()),
        Some("Stripe and PayPal".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    )?;

    // LIST all entries for the component
    list::handle(None, Some("payment.gateway".to_string()), false)?;

    // DELETE each entry type
    delete::handle(
        EntryType::Metadata,
        Some("payment.gateway".to_string()),
        None,
        false,
    )?;
    delete::handle(
        EntryType::Debug,
        Some("payment.gateway".to_string()),
        None,
        false,
    )?;
    delete::handle(
        EntryType::QA,
        Some("payment.gateway".to_string()),
        None,
        false,
    )?;

    // Verify all deleted
    assert!(!get_entry_path(&EntryType::Metadata, "payment.gateway").exists());
    assert!(!get_entry_path(&EntryType::Debug, "payment.gateway").exists());
    assert!(!get_entry_path(&EntryType::QA, "payment.gateway").exists());

    Ok(())
}

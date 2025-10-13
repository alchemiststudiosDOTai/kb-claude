use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;

use crate::agent::AgentResponse;
use crate::io::get_entry_path;

pub fn handle(
    entry_type: String,
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

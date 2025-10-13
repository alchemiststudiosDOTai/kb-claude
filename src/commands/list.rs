use anyhow::Result;
use colored::Colorize;
use serde_json::Value;

use crate::io::{list_entries, read_json};

pub fn handle(
    entry_type: Option<String>,
    component: Option<String>,
    json_output: bool,
) -> Result<()> {
    let paths = list_entries(entry_type.as_deref())?;

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

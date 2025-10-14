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
            let summary = summary
                .ok_or_else(|| anyhow!("--summary (file path) is required for code_index"))?;

            let mut code_index = if path.exists() {
                read_json(&path)?
            } else {
                CodeIndex::new(component.clone())
            };

            code_index.add_file(summary, error);
            write_json(&path, &code_index, true)?;
        }
        EntryType::Pattern => {
            let summary = summary
                .ok_or_else(|| anyhow!("--summary (pattern name) is required for pattern"))?;
            let error =
                error.ok_or_else(|| anyhow!("--error (description) is required for pattern"))?;

            let mut pattern = if path.exists() {
                read_json(&path)?
            } else {
                Pattern::new(component.clone())
            };

            pattern.add_pattern(summary, error, solution);
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

            if let (Some(heading), Some(content)) = (error, solution) {
                cheatsheet.add_section(heading, content);
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

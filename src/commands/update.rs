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
            return Err(anyhow!(
                "Delta entries are not supported via the update command"
            ));
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

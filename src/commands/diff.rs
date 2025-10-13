use anyhow::{anyhow, Result};
use colored::Colorize;
use serde_json::json;
use std::process::Command;

pub fn handle(since: Option<String>, json_output: bool) -> Result<()> {
    let git_ref = since.unwrap_or_else(|| "HEAD~1".to_string());

    let output = Command::new("git")
        .args(["diff", &git_ref, "HEAD", "--", ".claude/"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Git diff failed. Make sure you're in a git repository."
        ));
    }

    let diff = String::from_utf8_lossy(&output.stdout);

    if json_output {
        let lines: Vec<&str> = diff.lines().collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "since": git_ref,
                "changes": lines,
                "has_changes": !diff.is_empty()
            }))?
        );
    } else if diff.is_empty() {
        println!("{}", "No changes in .claude/ directory".yellow());
    } else {
        println!(
            "{}",
            format!("Changes since {}", git_ref).bold().underline()
        );
        println!();

        for line in diff.lines() {
            if line.starts_with('+') && !line.starts_with("+++") {
                println!("{}", line.green());
            } else if line.starts_with('-') && !line.starts_with("---") {
                println!("{}", line.red());
            } else if line.starts_with("@@") {
                println!("{}", line.cyan().bold());
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

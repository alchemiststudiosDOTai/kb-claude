use anyhow::Result;
use colored::Colorize;

use crate::io::ensure_claude_dirs;
use crate::manifest::synchronize;

pub fn handle(verbose: bool, json: bool) -> Result<()> {
    ensure_claude_dirs()?;

    let report = synchronize()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("{}", "Synchronizing manifest...".bold());
        println!();

        if !report.added.is_empty() {
            println!("{} {} files", "Added:".green().bold(), report.added.len());
            if verbose {
                for file in &report.added {
                    println!("  + {}", file);
                }
            }
        }

        if !report.updated.is_empty() {
            println!(
                "{} {} files",
                "Updated:".yellow().bold(),
                report.updated.len()
            );
            if verbose {
                for file in &report.updated {
                    println!("  ~ {}", file);
                }
            }
        }

        if !report.deleted.is_empty() {
            println!("{} {} files", "Deleted:".red().bold(), report.deleted.len());
            if verbose {
                for file in &report.deleted {
                    println!("  - {}", file);
                }
            }
        }

        if report.added.is_empty() && report.updated.is_empty() && report.deleted.is_empty() {
            println!("{}", "Everything is up to date ✓".green());
        }

        println!();
        println!(
            "Timestamp: {}",
            report.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );
    }

    Ok(())
}

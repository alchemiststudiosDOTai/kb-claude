use anyhow::Result;
use colored::Colorize;
use serde_json::json;

use crate::io::list_entries;
use crate::schema::validate_file;

pub fn handle(path: Option<String>, json_output: bool) -> Result<()> {
    let paths = if let Some(p) = path {
        vec![std::path::PathBuf::from(p)]
    } else {
        list_entries(None)?
    };

    if paths.is_empty() {
        if json_output {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "status": "success",
                    "message": "No files to validate",
                    "results": []
                }))?
            );
        } else {
            println!("{}", "No files to validate".yellow());
        }
        return Ok(());
    }

    let mut all_valid = true;
    let mut results = Vec::new();

    for path in paths {
        let result = validate_file(&path)?;

        if !result.valid {
            all_valid = false;
        }

        if json_output {
            results.push(json!({
                "file": result.file,
                "valid": result.valid,
                "errors": result.errors
            }));
        } else if result.valid {
            println!("{} {}", "✓".green().bold(), result.file);
        } else {
            println!("{} {}", "✗".red().bold(), result.file);
            for error in &result.errors {
                println!("    {}", error.red());
            }
        }
    }

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "status": if all_valid { "pass" } else { "fail" },
                "results": results
            }))?
        );
    } else {
        println!();
        if all_valid {
            println!("{}", "All files are valid ✓".green().bold());
        } else {
            println!("{}", "Validation failed ✗".red().bold());
        }
    }

    Ok(())
}

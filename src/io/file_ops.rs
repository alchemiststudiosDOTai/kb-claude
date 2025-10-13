use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const CLAUDE_DIR: &str = ".claude";

pub fn get_claude_path() -> PathBuf {
    PathBuf::from(CLAUDE_DIR)
}

pub fn ensure_claude_dirs() -> Result<()> {
    let base = get_claude_path();
    let dirs = vec![
        "metadata",
        "code_index",
        "debug_history",
        "patterns",
        "qa",
        "cheatsheets",
        "delta",
    ];

    if !base.exists() {
        fs::create_dir(&base).context("Failed to create .claude directory")?;
    }

    for dir in dirs {
        let path = base.join(dir);
        if !path.exists() {
            fs::create_dir_all(&path).context(format!("Failed to create {}", dir))?;
        }
    }

    Ok(())
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let content =
        fs::read_to_string(path).context(format!("Failed to read file: {}", path.display()))?;
    let data = serde_json::from_str(&content)
        .context(format!("Failed to parse JSON from: {}", path.display()))?;
    Ok(data)
}

pub fn write_json<T: Serialize>(path: &Path, data: &T, pretty: bool) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = if pretty {
        serde_json::to_string_pretty(data)?
    } else {
        serde_json::to_string(data)?
    };

    fs::write(path, content).context(format!("Failed to write file: {}", path.display()))?;
    Ok(())
}

pub fn get_entry_path(entry_type: &str, component: &str) -> PathBuf {
    let base = get_claude_path();
    let subdir = match entry_type {
        "metadata" => "metadata",
        "debug" => "debug_history",
        "qa" => "qa",
        "delta" => "delta",
        "code_index" => "code_index",
        "pattern" => "patterns",
        "cheatsheet" => "cheatsheets",
        _ => "metadata",
    };

    let filename = format!("{}.json", component);
    base.join(subdir).join(filename)
}

pub fn list_entries(entry_type: Option<&str>) -> Result<Vec<PathBuf>> {
    let base = get_claude_path();
    let mut entries = Vec::new();

    let dirs_to_scan = if let Some(t) = entry_type {
        vec![match t {
            "metadata" => "metadata",
            "debug" => "debug_history",
            "qa" => "qa",
            "delta" => "delta",
            "code_index" => "code_index",
            "pattern" => "patterns",
            _ => return Ok(entries),
        }]
    } else {
        vec![
            "metadata",
            "debug_history",
            "qa",
            "delta",
            "code_index",
            "patterns",
        ]
    };

    for dir in dirs_to_scan {
        let path = base.join(dir);
        if path.exists() && path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|e| e == "json") {
                    entries.push(path);
                }
            }
        }
    }

    Ok(entries)
}

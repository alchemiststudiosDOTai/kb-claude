use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::fs::{
    claude_root_from, display_relative, ClaudePaths, CLAUDE_DIRECTORIES, CURRENT_DIR_ERROR,
};

use super::InitArgs;

pub fn run(args: InitArgs) -> Result<()> {
    let workspace = normalize_workspace(&args.directory)?;
    let claude_root = claude_root_from(&workspace);
    let planned = plan_layout(&workspace, &claude_root);

    if args.dry_run {
        report_changes(&workspace, &claude_root, &planned, ReportMode::DryRun);
        return Ok(());
    }

    if !workspace.exists() {
        fs::create_dir_all(&workspace)
            .with_context(|| format!("Unable to create {}", workspace.display()))?;
    }

    let layout = ClaudePaths::new(claude_root.clone());
    layout.ensure_layout()?;

    report_changes(&workspace, &claude_root, &planned, ReportMode::Execution);

    Ok(())
}

fn normalize_workspace(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().context(CURRENT_DIR_ERROR)?;
    Ok(cwd.join(path))
}

fn plan_layout(workspace: &Path, claude_root: &Path) -> Vec<PathBuf> {
    let mut planned = Vec::new();

    if !workspace.exists() {
        planned.push(workspace.to_path_buf());
    }
    if !claude_root.exists() {
        planned.push(claude_root.to_path_buf());
    }
    for directory in CLAUDE_DIRECTORIES {
        let path = claude_root.join(directory);
        if !path.exists() {
            planned.push(path);
        }
    }

    planned
}

enum ReportMode {
    DryRun,
    Execution,
}

fn report_changes(workspace: &Path, claude_root: &Path, planned: &[PathBuf], mode: ReportMode) {
    let (empty_msg, header_msg, prefix) = match mode {
        ReportMode::DryRun => (
            "Dry run: .claude hierarchy already exists at",
            "Dry run: would initialize .claude hierarchy under",
            "  + ",
        ),
        ReportMode::Execution => (
            "No changes needed; .claude hierarchy already present at",
            "Initialized .claude hierarchy under",
            "  created ",
        ),
    };

    if planned.is_empty() {
        println!("{} {}", empty_msg, claude_root.display());
        return;
    }

    println!("{} {}", header_msg, claude_root.display());
    for path in planned {
        println!("{}{}", prefix, display_relative(workspace, path));
    }
}

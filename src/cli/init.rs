use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::fs::{claude_root_from, ClaudePaths, CLAUDE_DIRECTORIES};

use super::InitArgs;

pub fn run(args: InitArgs) -> Result<()> {
    let workspace = normalize_workspace(&args.directory)?;
    let claude_root = claude_root_from(&workspace);
    let planned = plan_layout(&workspace, &claude_root);

    if args.dry_run {
        report_dry_run(&workspace, &claude_root, &planned);
        return Ok(());
    }

    if !workspace.exists() {
        fs::create_dir_all(&workspace)
            .with_context(|| format!("Unable to create {}", workspace.display()))?;
    }

    let layout = ClaudePaths::new(claude_root.clone());
    layout.ensure_layout()?;

    report_execution(&workspace, &claude_root, &planned);

    Ok(())
}

fn normalize_workspace(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().context("Unable to determine current directory")?;
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

fn report_dry_run(workspace: &Path, claude_root: &Path, planned: &[PathBuf]) {
    if planned.is_empty() {
        println!(
            "Dry run: .claude hierarchy already exists at {}",
            claude_root.display()
        );
        return;
    }

    println!(
        "Dry run: would initialize .claude hierarchy under {}",
        claude_root.display()
    );
    for path in planned {
        println!("  + {}", display_relative(workspace, path));
    }
}

fn report_execution(workspace: &Path, claude_root: &Path, planned: &[PathBuf]) {
    if planned.is_empty() {
        println!(
            "No changes needed; .claude hierarchy already present at {}",
            claude_root.display()
        );
        return;
    }

    println!(
        "Initialized .claude hierarchy under {}",
        claude_root.display()
    );
    for path in planned {
        println!("  created {}", display_relative(workspace, path));
    }
}

fn display_relative<'a>(workspace: &Path, path: &'a Path) -> Cow<'a, str> {
    if path == workspace {
        return Cow::Borrowed(".");
    }

    match path.strip_prefix(workspace) {
        Ok(relative) if relative.as_os_str().is_empty() => Cow::Borrowed("."),
        Ok(relative) => Cow::Owned(format!("./{}", relative.display())),
        Err(_) => Cow::Owned(path.display().to_string()),
    }
}

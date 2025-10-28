use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use walkdir::WalkDir;

use super::ManifestArgs;
use crate::fs::{claude_root_from, find_existing_root, ClaudePaths};
use crate::model::Document;

pub fn run(args: ManifestArgs) -> Result<()> {
    let cwd = std::env::current_dir().context("Unable to determine current directory")?;
    let base_dir = args.directory.as_deref().unwrap_or(&cwd);
    let claude_root = find_existing_root(base_dir).unwrap_or_else(|| claude_root_from(base_dir));

    if !claude_root.exists() {
        bail!(
            "No .claude directory found under {}. Run `kb-claude init` first.",
            base_dir.display()
        );
    }

    let layout = ClaudePaths::new(claude_root.clone());
    let entries = collect_entries(&claude_root)?;

    let manifest_content = render_manifest(&claude_root, &entries)?;
    let output_path = resolve_output_path(&cwd, &layout, args.output.as_ref())?;

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Unable to prepare {}", parent.display()))?;
    }

    fs::write(&output_path, manifest_content)
        .with_context(|| format!("Unable to write {}", output_path.display()))?;

    println!("Wrote manifest to {}", output_path.display());
    Ok(())
}

#[derive(Debug)]
struct ManifestEntry {
    title: String,
    doc_type: String,
    relative_path: PathBuf,
    tags: Vec<String>,
    relations: Vec<String>,
    updated_at: chrono::NaiveDate,
}

fn collect_entries(claude_root: &Path) -> Result<Vec<ManifestEntry>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(claude_root) {
        let entry = entry?;
        let path = entry.path();

        if !entry.file_type().is_file() {
            continue;
        }

        if path
            .file_name()
            .is_some_and(|name| name == crate::fs::MANIFEST_FILE)
        {
            continue;
        }

        if crate::fs::is_ignored_path(path, claude_root) {
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Unable to read {}", path.display()))?;
        let document = Document::parse(&content)
            .with_context(|| format!("Unable to parse {}", path.display()))?;

        let relative = path
            .strip_prefix(claude_root.parent().unwrap_or(claude_root))
            .unwrap_or(path)
            .to_path_buf();

        entries.push(ManifestEntry {
            title: document.front_matter.title.clone(),
            doc_type: document.front_matter.doc_type.clone(),
            relative_path: relative,
            tags: document.front_matter.tags.clone(),
            relations: document
                .front_matter
                .ontological_relations
                .iter()
                .map(|relation| relation.relates_to.clone())
                .collect(),
            updated_at: document.front_matter.updated_at.date_naive(),
        });
    }

    entries.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    Ok(entries)
}

fn render_manifest(claude_root: &Path, entries: &[ManifestEntry]) -> Result<String> {
    let mut lines = Vec::new();
    lines.push("| Title | Type | Path | Tags | Relations | Updated |".to_string());
    lines.push("|-------|------|------|------|-----------|---------|".to_string());

    for entry in entries {
        let path_display = format_path(claude_root, &entry.relative_path);
        let tags = format_list(&entry.tags);
        let relations = format_list(&entry.relations);
        let updated = entry.updated_at.to_string();

        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            entry.title, entry.doc_type, path_display, tags, relations, updated
        ));
    }

    if entries.is_empty() {
        lines.push("| *(empty)* | - | - | - | - | - |".to_string());
    }

    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn format_path(claude_root: &Path, relative_path: &Path) -> String {
    let path = if relative_path.is_relative() {
        relative_path.to_path_buf()
    } else {
        relative_path
            .strip_prefix(claude_root.parent().unwrap_or(claude_root))
            .unwrap_or(relative_path)
            .to_path_buf()
    };
    format!("./{}", path.display())
}

fn format_list(values: &[String]) -> String {
    if values.is_empty() {
        "—".to_string()
    } else {
        values.join(", ")
    }
}

fn resolve_output_path(
    cwd: &Path,
    layout: &ClaudePaths,
    override_path: Option<&PathBuf>,
) -> Result<PathBuf> {
    if let Some(custom) = override_path {
        if custom.is_absolute() {
            return Ok(custom.clone());
        }
        return Ok(cwd.join(custom));
    }

    Ok(layout.manifest_path())
}

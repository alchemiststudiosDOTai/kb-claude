use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::ManifestArgs;
use crate::fs::{resolve_claude_root, walk_kb_documents, ClaudePaths};

pub fn run(args: ManifestArgs) -> Result<()> {
    let (base_dir, claude_root) = resolve_claude_root(args.directory.as_deref())?;

    if !claude_root.exists() {
        bail!(
            "No .claude directory found under {}. Run `kb-claude init` first.",
            base_dir.display()
        );
    }

    let layout = ClaudePaths::new(claude_root.clone());
    let entries = collect_entries(&claude_root)?;

    let manifest_content = render_manifest(&claude_root, &entries)?;
    let output_path = resolve_output_path(&base_dir, &layout, args.output.as_ref())?;

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

    for entry_result in walk_kb_documents(claude_root) {
        let entry = entry_result?;
        let relative = entry
            .path
            .strip_prefix(claude_root.parent().unwrap_or(claude_root))
            .unwrap_or(&entry.path)
            .to_path_buf();

        entries.push(ManifestEntry {
            title: entry.document.front_matter.title.clone(),
            doc_type: entry.document.front_matter.doc_type.clone(),
            relative_path: relative,
            tags: entry.document.front_matter.tags.clone(),
            relations: entry
                .document
                .front_matter
                .ontological_relations
                .iter()
                .map(|relation| relation.relates_to.clone())
                .collect(),
            updated_at: entry.document.front_matter.updated_at.date_naive(),
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

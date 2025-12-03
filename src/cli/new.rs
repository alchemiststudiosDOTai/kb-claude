use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};

use super::NewArgs;
use crate::fs::{display_relative, resolve_claude_root_from_cwd, ClaudePaths};
use crate::model::{Document, DocumentFrontMatter, OntologicalRelation};

pub fn run(args: NewArgs) -> Result<()> {
    let (cwd, claude_root) = resolve_claude_root_from_cwd()?;
    let workspace = claude_root
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| cwd.clone());

    let layout = ClaudePaths::new(claude_root.clone());
    if !claude_root.exists() {
        layout.ensure_layout()?;
        println!(
            "No existing knowledge base detected; created layout at {}",
            claude_root.display()
        );
    }

    let doc_type = determine_type(&layout, args.doc_type.as_deref())?;
    let tags = collect_tags(&args)?;
    let relations = collect_relations(&args)?;
    let body = collect_body()?;

    let mut front_matter = DocumentFrontMatter::new(&args.title, doc_type);
    front_matter.tags = tags;
    front_matter.ontological_relations = relations;
    front_matter.ensure_link_matches_slug();

    let output_path = compute_output_path(
        &cwd,
        &layout,
        &mut front_matter,
        args.file_override.as_ref(),
    )?;
    ensure_parent_dirs(&output_path)?;

    if output_path.exists() {
        bail!(
            "A document already exists at {}; choose a different title or override path",
            output_path.display()
        );
    }

    let document = Document::new(front_matter, body);
    let content = document.to_markdown()?;
    fs::write(&output_path, content)
        .with_context(|| format!("Unable to write {}", output_path.display()))?;

    println!("Created {}", display_relative(&workspace, &output_path));

    Ok(())
}

fn determine_type(layout: &ClaudePaths, provided: Option<&str>) -> Result<String> {
    if let Some(doc_type) = provided {
        validate_type(layout, doc_type)?;
        return Ok(doc_type.to_string());
    }

    let known = layout.known_types();
    loop {
        println!("Select type:");
        for entry in known {
            println!("  - {}", entry);
        }
        print!("Type [{}]: ", known.first().copied().unwrap_or_default());
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let candidate = input.trim();
        let value = if candidate.is_empty() {
            known.first().copied().unwrap_or_default()
        } else {
            candidate
        };

        if validate_type(layout, value).is_ok() {
            return Ok(value.to_string());
        }

        println!("Invalid type `{value}`; please choose one of the listed options.");
    }
}

fn validate_type(layout: &ClaudePaths, doc_type: &str) -> Result<()> {
    if layout.is_supported_type(doc_type) {
        Ok(())
    } else {
        bail!(
            "Unsupported type `{doc_type}`. Expected one of: {}",
            layout.known_types().join(", ")
        )
    }
}

fn collect_tags(args: &NewArgs) -> Result<Vec<String>> {
    if !args.tags.is_empty() {
        return Ok(args.tags.iter().map(|t| t.trim().to_string()).collect());
    }

    print!("Tags (comma separated, optional): ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let tags = input
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect();
    Ok(tags)
}

fn collect_relations(args: &NewArgs) -> Result<Vec<OntologicalRelation>> {
    if !args.relates_to.is_empty() {
        return Ok(args
            .relates_to
            .iter()
            .map(|link| OntologicalRelation {
                relates_to: link.trim().to_string(),
            })
            .collect());
    }

    print!("Relates to (comma separated slugs, optional): ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let relations = input
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| OntologicalRelation {
            relates_to: value.to_string(),
        })
        .collect();
    Ok(relations)
}

fn collect_body() -> Result<String> {
    println!("Body (finish with an empty line):");
    let mut lines = Vec::new();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let trimmed = buffer.trim_end_matches(['\n', '\r']);
        if trimmed.is_empty() {
            break;
        }
        lines.push(trimmed.to_string());
    }
    Ok(lines.join("\n"))
}

fn compute_output_path(
    cwd: &Path,
    layout: &ClaudePaths,
    front_matter: &mut DocumentFrontMatter,
    override_path: Option<&PathBuf>,
) -> Result<PathBuf> {
    if let Some(custom) = override_path {
        let path = if custom.is_relative() {
            cwd.join(custom)
        } else {
            custom.clone()
        };
        ensure_markdown_extension(&path)?;
        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            let slug = crate::model::slugify(stem);
            front_matter.link = slug;
        }
        return Ok(path);
    }

    let mut path = layout.type_directory(&front_matter.doc_type);
    path.push(format!("{}.md", front_matter.link));
    Ok(path)
}

fn ensure_markdown_extension(path: &Path) -> Result<()> {
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
    {
        Ok(())
    } else {
        Err(anyhow!(
            "Expected markdown (.md) extension for {}; adjust the file override",
            path.display()
        ))
    }
}

fn ensure_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Unable to create {}", parent.display()))?;
    }
    Ok(())
}

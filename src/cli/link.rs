use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use walkdir::WalkDir;

use super::LinkArgs;
use crate::fs::{claude_root_from, find_existing_root};
use crate::model::{Document, OntologicalRelation};

pub fn run(args: LinkArgs) -> Result<()> {
    if args.source == args.target {
        bail!("Source and target must be different links.");
    }

    let cwd = std::env::current_dir().context("Unable to determine current directory")?;
    let claude_root = find_existing_root(&cwd).unwrap_or_else(|| claude_root_from(&cwd));

    if !claude_root.exists() {
        bail!(
            "No .claude directory found under {}. Run `kb-claude init` first.",
            cwd.display()
        );
    }

    let mut source = load_document(&claude_root, &args.source)?;
    let mut target = load_document(&claude_root, &args.target)?;

    let added_source = insert_relation(
        &mut source.document,
        &target.document.front_matter.link,
        args.force,
    );
    let added_target = insert_relation(
        &mut target.document,
        &source.document.front_matter.link,
        args.force,
    );

    if !added_source && !added_target && !args.force {
        println!(
            "Relations already existed between `{}` and `{}`; no changes made.",
            args.source, args.target
        );
        return Ok(());
    }

    let source_path = source.path.clone();
    let target_path = target.path.clone();
    write_document(&source)?;
    write_document(&target)?;

    println!(
        "Linked {} <-> {}",
        display_relative(&cwd, &source_path),
        display_relative(&cwd, &target_path)
    );

    Ok(())
}

struct DocumentRecord {
    path: PathBuf,
    document: Document,
}

fn load_document(claude_root: &Path, slug: &str) -> Result<DocumentRecord> {
    let mut matches = Vec::new();
    for entry in WalkDir::new(claude_root) {
        let entry = entry?;
        let path = entry.path();

        if !entry.file_type().is_file() {
            continue;
        }

        if crate::fs::is_ignored_path(path, claude_root) {
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        if stem != slug {
            continue;
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Unable to read {}", path.display()))?;
        let document = Document::parse(&content)
            .with_context(|| format!("Unable to parse {}", path.display()))?;

        if document.front_matter.link != slug {
            continue;
        }

        matches.push(DocumentRecord {
            path: path.to_path_buf(),
            document,
        });
    }

    match matches.len() {
        0 => bail!("No document found with link `{slug}`."),
        1 => Ok(matches.remove(0)),
        _ => bail!("Multiple documents found with link `{slug}`; please disambiguate."),
    }
}

fn insert_relation(document: &mut Document, target_link: &str, force: bool) -> bool {
    let relations = &mut document.front_matter.ontological_relations;
    let exists = relations
        .iter()
        .any(|relation| relation.relates_to == target_link);

    if exists && !force {
        return false;
    }

    if !exists || force {
        relations.push(OntologicalRelation {
            relates_to: target_link.to_string(),
        });
        document.front_matter.touch_updated();
        return true;
    }

    false
}

fn write_document(record: &DocumentRecord) -> Result<()> {
    let content = record.document.to_markdown()?;
    fs::write(&record.path, content)
        .with_context(|| format!("Unable to write {}", record.path.display()))?;
    Ok(())
}

fn display_relative(cwd: &Path, path: &Path) -> String {
    path.strip_prefix(cwd)
        .map(|relative| format!("./{}", relative.display()))
        .unwrap_or_else(|_| path.display().to_string())
}

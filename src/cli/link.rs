use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::LinkArgs;
use crate::fs::{display_relative, resolve_claude_root_from_cwd, walk_kb_documents};
use crate::model::{Document, OntologicalRelation};

pub fn run(args: LinkArgs) -> Result<()> {
    if args.source == args.target {
        bail!("Source and target must be different links.");
    }

    let (cwd, claude_root) = resolve_claude_root_from_cwd()?;

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

    for entry_result in walk_kb_documents(claude_root) {
        let entry = entry_result?;
        let path = &entry.path;

        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        if stem != slug {
            continue;
        }

        if entry.document.front_matter.link != slug {
            continue;
        }

        matches.push(DocumentRecord {
            path: path.clone(),
            document: entry.document,
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

    // Guard: early return if relation exists and not forcing
    if exists && !force {
        return false;
    }

    // Only add if doesn't exist or we're forcing
    if !exists || force {
        relations.push(OntologicalRelation {
            relates_to: target_link.to_string(),
        });
        document.front_matter.touch_updated();
    }

    true
}

fn write_document(record: &DocumentRecord) -> Result<()> {
    let content = record.document.to_markdown()?;
    fs::write(&record.path, content)
        .with_context(|| format!("Unable to write {}", record.path.display()))?;
    Ok(())
}

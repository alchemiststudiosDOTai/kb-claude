use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use walkdir::WalkDir;

use super::SearchArgs;
use crate::fs::{claude_root_from, find_existing_root};
use crate::model::Document;

pub fn run(args: SearchArgs) -> Result<()> {
    let cwd = std::env::current_dir().context("Unable to determine current directory")?;
    let claude_root = find_existing_root(&cwd).unwrap_or_else(|| claude_root_from(&cwd));

    if !claude_root.exists() {
        bail!(
            "No .claude directory found under {}. Run `kb-claude init` first.",
            cwd.display()
        );
    }

    let terms: Vec<String> = args.terms.iter().map(|term| term.to_lowercase()).collect();
    let tag_filters: Vec<String> = args.tags.iter().map(|tag| tag.to_lowercase()).collect();

    let documents = collect_documents(&claude_root)?;
    let mut matches: Vec<SearchMatch> = documents
        .into_iter()
        .filter_map(|entry| filter_match(&claude_root, &entry, &terms, &tag_filters))
        .collect();

    matches.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.title.cmp(&b.title)));

    if matches.is_empty() {
        println!("No matching entries found.");
        return Ok(());
    }

    for (index, item) in matches.iter().enumerate() {
        println!(
            "{}. {} — {} (type: {}, tags: {})",
            index + 1,
            item.path.display(),
            item.title,
            item.doc_type,
            if item.tags.is_empty() {
                "—".to_string()
            } else {
                item.tags.join(", ")
            }
        );
    }

    Ok(())
}

#[derive(Debug)]
struct DocumentEntry {
    path: PathBuf,
    document: Document,
}

#[derive(Debug)]
struct SearchMatch {
    title: String,
    doc_type: String,
    tags: Vec<String>,
    path: PathBuf,
    score: usize,
}

fn collect_documents(claude_root: &Path) -> Result<Vec<DocumentEntry>> {
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

        entries.push(DocumentEntry {
            path: path.to_path_buf(),
            document,
        });
    }

    Ok(entries)
}

fn filter_match(
    claude_root: &Path,
    entry: &DocumentEntry,
    terms: &[String],
    tag_filters: &[String],
) -> Option<SearchMatch> {
    let front = &entry.document.front_matter;

    if !tag_filters.is_empty() {
        let tag_set: Vec<String> = front.tags.iter().map(|tag| tag.to_lowercase()).collect();
        if !tag_filters
            .iter()
            .all(|needle| tag_set.iter().any(|tag| tag == needle))
        {
            return None;
        }
    }

    let searchable = build_search_blob(front, &entry.document.body);

    let mut score = 0;
    for term in terms {
        if !searchable.contains(term) {
            return None;
        }
        score += searchable.matches(term).count();
    }

    let relative = entry
        .path
        .strip_prefix(claude_root.parent().unwrap_or(claude_root))
        .unwrap_or(&entry.path);

    Some(SearchMatch {
        title: front.title.clone(),
        doc_type: front.doc_type.clone(),
        tags: front.tags.clone(),
        path: PathBuf::from(format!("./{}", relative.display())),
        score,
    })
}

fn build_search_blob(front: &crate::model::DocumentFrontMatter, body: &str) -> String {
    let mut blob = vec![
        front.title.to_lowercase(),
        front.link.to_lowercase(),
        front.doc_type.to_lowercase(),
        body.to_lowercase(),
    ];
    if !front.tags.is_empty() {
        blob.push(front.tags.join(" ").to_lowercase());
    }
    if !front.ontological_relations.is_empty() {
        let relations = front
            .ontological_relations
            .iter()
            .map(|rel| rel.relates_to.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");
        blob.push(relations);
    }
    blob.join(" ")
}

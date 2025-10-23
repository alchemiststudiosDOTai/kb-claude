use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use walkdir::WalkDir;

use super::ValidateArgs;
use crate::fs::{claude_root_from, find_existing_root, ClaudePaths};
use crate::model::Document;

pub fn run(args: ValidateArgs) -> Result<()> {
    let cwd = std::env::current_dir().context("Unable to determine current directory")?;
    let target_dir = args.directory.as_deref().unwrap_or(&cwd);
    let claude_root =
        find_existing_root(target_dir).unwrap_or_else(|| claude_root_from(target_dir));

    if !claude_root.exists() {
        bail!(
            "No .claude directory found under {}. Run `kb-claude init` first.",
            target_dir.display()
        );
    }

    let workspace = claude_root
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| target_dir.to_path_buf());

    let layout = ClaudePaths::new(claude_root.clone());
    let findings = collect_findings(&claude_root, &layout)?;

    if findings.is_empty() {
        println!(
            "Validated .claude hierarchy at {}; no issues found.",
            claude_root.display()
        );
        return Ok(());
    }

    print_findings(&findings, &workspace);

    let error_count = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Error)
        .count();
    let warning_count = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Warning)
        .count();

    if error_count > 0 || (args.strict && warning_count > 0) {
        let mut parts = Vec::new();
        if error_count > 0 {
            parts.push(format!("{error_count} error(s)"));
        }
        if args.strict && warning_count > 0 {
            parts.push(format!("{warning_count} warning(s)"));
        }
        bail!("Validation failed with {}.", parts.join(" and "));
    }

    println!(
        "Validation completed with {warning_count} warning(s). Run with --strict to fail on warnings."
    );
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
struct Finding {
    path: PathBuf,
    message: String,
    severity: Severity,
}

fn collect_findings(claude_root: &Path, layout: &ClaudePaths) -> Result<Vec<Finding>> {
    let mut findings = Vec::new();

    for entry in WalkDir::new(claude_root) {
        let entry = entry?;
        let path = entry.path();

        if path != claude_root && is_hidden(path) {
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }

        if path
            .file_name()
            .is_some_and(|name| name == crate::fs::MANIFEST_FILE)
        {
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(value) => value,
            Err(error) => {
                findings.push(Finding {
                    path: path.to_path_buf(),
                    message: format!("Unable to read file: {error}"),
                    severity: Severity::Error,
                });
                continue;
            }
        };

        let document = match Document::parse(&content) {
            Ok(doc) => doc,
            Err(error) => {
                findings.push(Finding {
                    path: path.to_path_buf(),
                    message: format!("Invalid front matter: {error:#}"),
                    severity: Severity::Error,
                });
                continue;
            }
        };

        let errors = validate_document(path, claude_root, layout, &document)?;
        findings.extend(errors);
    }

    Ok(findings)
}

fn validate_document(
    path: &Path,
    claude_root: &Path,
    layout: &ClaudePaths,
    document: &Document,
) -> Result<Vec<Finding>> {
    let mut findings = Vec::new();
    let front = &document.front_matter;

    if front.title.trim().is_empty() {
        findings.push(error(path, "Missing `title`"));
    }

    if front.link.trim().is_empty() {
        findings.push(error(path, "Missing `link`"));
    }

    if front.doc_type.trim().is_empty() {
        findings.push(error(path, "Missing `type`"));
    }

    if front.uuid.as_bytes().iter().all(|byte| *byte == 0) {
        findings.push(error(path, "`uuid` cannot be nil"));
    }

    if !layout.is_supported_type(&front.doc_type) {
        findings.push(error(
            path,
            &format!(
                "`type` `{}` is not one of the configured directories",
                front.doc_type
            ),
        ));
    }

    let file_stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| anyhow!("Unable to determine file name for {}", path.display()))?;

    if front.link != file_stem {
        findings.push(warning(
            path,
            &format!(
                "`link` `{}` should match file name `{}`",
                front.link, file_stem
            ),
        ));
    }

    if !front.is_link_consistent() {
        findings.push(warning(
            path,
            &format!(
                "`link` `{}` should match slugified title `{}`",
                front.link,
                front.slug_from_title()
            ),
        ));
    }

    if let Some(type_dir) = type_directory_name(path, claude_root) {
        if type_dir != front.doc_type {
            findings.push(error(
                path,
                &format!(
                    "Stored under type directory `{type_dir}` but front matter type is `{}`",
                    front.doc_type
                ),
            ));
        }
    }

    Ok(findings)
}

fn is_hidden(path: &Path) -> bool {
    if let Some(parent) = path.parent() {
        if parent
            .file_name()
            .is_some_and(|name| name == crate::fs::CLAUDE_ROOT)
        {
            // allow standard directories inside .claude even if they start with dot (unlikely)
            return false;
        }
    }
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn type_directory_name(path: &Path, claude_root: &Path) -> Option<String> {
    let relative = path.strip_prefix(claude_root).ok()?;
    let mut components = relative.components();
    components
        .next()
        .and_then(|component| component.as_os_str().to_str().map(ToString::to_string))
}

fn print_findings(findings: &[Finding], workspace: &Path) {
    for finding in findings {
        let label = match finding.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let display = finding
            .path
            .strip_prefix(workspace)
            .map(|path| format!("./{}", path.display()))
            .unwrap_or_else(|_| finding.path.display().to_string());
        println!("{label}: {display} — {}", finding.message);
    }
}

fn error(path: &Path, message: &str) -> Finding {
    Finding {
        path: path.to_path_buf(),
        message: message.to_string(),
        severity: Severity::Error,
    }
}

fn warning(path: &Path, message: &str) -> Finding {
    Finding {
        path: path.to_path_buf(),
        message: message.to_string(),
        severity: Severity::Warning,
    }
}

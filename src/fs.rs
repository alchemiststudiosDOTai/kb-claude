use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result};

pub const CLAUDE_ROOT: &str = ".claude";
pub const MANIFEST_FILE: &str = "manifest.md";
pub const CLAUDE_DIRECTORIES: &[&str] = &[
    "metadata",
    "debug_history",
    "qa",
    "code_index",
    "patterns",
    "plans",
    "cheatsheets",
    "memory_anchors",
];

// Common error messages
pub const CURRENT_DIR_ERROR: &str = "Unable to determine current directory";
pub const NO_CLAUDE_DIR_ERROR: &str = "No .claude directory found under {}. Run `kb-claude init` first.";

// File extensions
pub const MD_EXTENSION: &str = "md";

#[derive(Debug, Clone)]
pub struct ClaudePaths {
    root: PathBuf,
}

impl ClaudePaths {
    pub fn new(base: impl AsRef<Path>) -> Self {
        Self {
            root: base.as_ref().to_path_buf(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.root.join(MANIFEST_FILE)
    }

    pub fn type_directory(&self, doc_type: &str) -> PathBuf {
        self.root.join(doc_type)
    }

    pub fn known_types(&self) -> &'static [&'static str] {
        CLAUDE_DIRECTORIES
    }

    pub fn is_supported_type(&self, doc_type: &str) -> bool {
        CLAUDE_DIRECTORIES.contains(&doc_type)
    }

    pub fn ensure_layout(&self) -> Result<()> {
        fs::create_dir_all(&self.root)
            .with_context(|| format!("Unable to create {}", self.root.display()))?;
        for directory in CLAUDE_DIRECTORIES {
            let target = self.root.join(directory);
            fs::create_dir_all(&target)
                .with_context(|| format!("Unable to create {}", target.display()))?;
        }
        Ok(())
    }
}

pub fn is_ignored_path(path: &Path, claude_root: &Path) -> bool {
    if let Ok(relative) = path.strip_prefix(claude_root) {
        if let Some(Component::Normal(component)) = relative.components().next() {
            if let Some(name) = component.to_str() {
                return !CLAUDE_DIRECTORIES.contains(&name);
            }
        }
    }
    false
}

pub fn claude_root_from(base: impl AsRef<Path>) -> PathBuf {
    base.as_ref().join(CLAUDE_ROOT)
}

pub fn find_existing_root(start: impl AsRef<Path>) -> Option<PathBuf> {
    let mut current = start.as_ref();
    loop {
        let candidate = current.join(CLAUDE_ROOT);
        if candidate.is_dir() {
            return Some(candidate);
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Resolves the claude root directory from the current working directory.
/// Returns both the current working directory and the claude root path.
pub fn resolve_claude_root_from_cwd() -> Result<(PathBuf, PathBuf)> {
    let cwd = std::env::current_dir().context(CURRENT_DIR_ERROR)?;
    let claude_root = find_existing_root(&cwd).unwrap_or_else(|| claude_root_from(&cwd));
    Ok((cwd, claude_root))
}

/// Displays a path relative to a workspace directory.
/// Returns a string like "./{relative_path}" for paths under workspace,
/// or the absolute path if not under workspace.
pub fn display_relative(workspace: &Path, path: &Path) -> String {
    match path.strip_prefix(workspace) {
        Ok(relative) if relative.as_os_str().is_empty() => ".".to_string(),
        Ok(relative) => format!("./{}", relative.display()),
        Err(_) => path.display().to_string(),
    }
}

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub const CLAUDE_ROOT: &str = ".claude";
pub const MANIFEST_FILE: &str = "manifest.md";
pub const CLAUDE_DIRECTORIES: &[&str] = &[
    "metadata",
    "debug_history",
    "qa",
    "code_index",
    "patterns",
    "cheatsheets",
    "memory_anchors",
];

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

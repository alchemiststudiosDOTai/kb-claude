use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::io::{get_claude_path, read_json, write_json};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub files: HashMap<String, FileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub hash: String,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReport {
    pub added: Vec<String>,
    pub updated: Vec<String>,
    pub deleted: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            timestamp: Utc::now(),
            files: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let path = get_manifest_path();
        if path.exists() {
            read_json(&path)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = get_manifest_path();
        write_json(&path, self, true)
    }
}

pub fn get_manifest_path() -> PathBuf {
    get_claude_path().join("manifest.json")
}

pub fn compute_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    Ok(hex::encode(hasher.finalize()))
}

pub fn scan_claude_files() -> Result<HashMap<String, FileEntry>> {
    let mut files = HashMap::new();
    let base = get_claude_path();

    if !base.exists() {
        return Ok(files);
    }

    for entry in WalkDir::new(&base)
        .into_iter()
        .filter_entry(|e| e.file_name() != "manifest.json")
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|e| e == "json" || e == "md") {
            let relative_path = path.strip_prefix(&base)?.to_string_lossy().to_string();

            let hash = compute_file_hash(path)?;
            let metadata = fs::metadata(path)?;
            let last_modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?;
            let last_modified = DateTime::from_timestamp(last_modified.as_secs() as i64, 0)
                .unwrap_or_else(Utc::now);

            files.insert(
                relative_path,
                FileEntry {
                    hash,
                    last_modified,
                },
            );
        }
    }

    Ok(files)
}

pub fn synchronize() -> Result<SyncReport> {
    let mut manifest = Manifest::load()?;
    let current_files = scan_claude_files()?;

    let mut report = SyncReport {
        added: Vec::new(),
        updated: Vec::new(),
        deleted: Vec::new(),
        timestamp: Utc::now(),
    };

    for (path, entry) in &current_files {
        if let Some(existing) = manifest.files.get(path) {
            if existing.hash != entry.hash {
                report.updated.push(path.clone());
            }
        } else {
            report.added.push(path.clone());
        }
    }

    for path in manifest.files.keys() {
        if !current_files.contains_key(path) {
            report.deleted.push(path.clone());
        }
    }

    manifest.files = current_files;
    manifest.timestamp = Utc::now();
    manifest.save()?;

    Ok(report)
}

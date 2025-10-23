use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const FRONT_MATTER_DELIMITER: &str = "---";
const FRONT_MATTER_BLOCK_BREAK: &str = "\n---\n";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OntologicalRelation {
    pub relates_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentFrontMatter {
    pub title: String,
    pub link: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    #[serde(default)]
    pub ontological_relations: Vec<OntologicalRelation>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(with = "iso8601")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "iso8601")]
    pub updated_at: DateTime<Utc>,
    pub uuid: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub front_matter: DocumentFrontMatter,
    pub body: String,
}

impl DocumentFrontMatter {
    pub fn new(title: impl Into<String>, doc_type: impl Into<String>) -> Self {
        let title_str = title.into();
        let doc_type_str = doc_type.into();
        let link = slugify(&title_str);
        let now = Utc::now();
        Self {
            title: title_str,
            link,
            doc_type: doc_type_str,
            ontological_relations: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn touch_updated(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn ensure_link_matches_slug(&mut self) {
        self.link = self.slug_from_title();
    }

    pub fn slug_from_title(&self) -> String {
        slugify(&self.title)
    }

    pub fn is_link_consistent(&self) -> bool {
        self.link == self.slug_from_title()
    }
}

impl Document {
    pub fn new(front_matter: DocumentFrontMatter, body: impl Into<String>) -> Self {
        Self {
            front_matter,
            body: body.into(),
        }
    }

    pub fn parse(raw: &str) -> Result<Self> {
        let trimmed = raw.trim_start();
        let rest = trimmed
            .strip_prefix(FRONT_MATTER_DELIMITER)
            .ok_or_else(|| anyhow!("Document missing starting front matter delimiter"))?;

        let rest = rest
            .strip_prefix('\n')
            .ok_or_else(|| anyhow!("Front matter must start on a new line"))?;

        let (yaml_block, body) = rest
            .split_once(FRONT_MATTER_BLOCK_BREAK)
            .ok_or_else(|| anyhow!("Document missing closing front matter delimiter"))?;

        let front_matter: DocumentFrontMatter = serde_yaml::from_str(yaml_block)
            .with_context(|| "Unable to parse document front matter as YAML")?;

        Ok(Self {
            front_matter,
            body: body.to_string(),
        })
    }

    pub fn to_markdown(&self) -> Result<String> {
        let yaml = serde_yaml::to_string(&self.front_matter)
            .with_context(|| "Unable to serialize document front matter")?;
        let yaml_trimmed = yaml.trim_start_matches(&format!("{FRONT_MATTER_DELIMITER}\n"));
        Ok(format!(
            "{delim}\n{front}{delim}\n{body}\n",
            delim = FRONT_MATTER_DELIMITER,
            front = yaml_trimmed,
            body = self.body.trim_end()
        ))
    }
}

pub fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;

    for ch in input.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            pending_dash = false;
        } else if matches!(ch, ' ' | '-' | '_' | '.') && !slug.is_empty() && !pending_dash {
            slug.push('-');
            pending_dash = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "untitled".to_string()
    } else {
        slug
    }
}

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn now_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub mod iso8601 {
    use std::str::FromStr;

    use chrono::{DateTime, SecondsFormat, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = value.to_rfc3339_opts(SecondsFormat::Secs, true);
        serializer.serialize_str(&formatted)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        DateTime::from_str(&raw).map_err(serde::de::Error::custom)
    }
}

use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    deleted: bool,
    #[serde(rename = "lastModified")]
    last_modified: String,
    #[serde(rename = "lastOpenedPage", default)]
    last_opened_page: u32,
    metadatamodified: bool,
    modified: bool,

    pub parent: String,
    pinned: bool,
    synced: bool,
    #[serde(rename = "type")]
    pub typ: String,
    version: u32,
    #[serde(rename = "visibleName")]
    pub visible_name: String,
}

impl Metadata {
    pub fn with_name_and_parent(name: impl AsRef<str>, parent: impl AsRef<str>) -> Metadata {
        Metadata {
            parent: parent.as_ref().into(),
            visible_name: name.as_ref().into(),
            ..Default::default()
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Metadata> {
        let md_path = path.as_ref().with_extension(super::METADATA_EXTENSION);
        let file = std::fs::File::open(&md_path)?;
        let metadata = serde_json::from_reader(file)?;

        Ok(metadata)
    }
}

impl Default for Metadata {
    fn default() -> Self {
        let time = std::time::SystemTime::now();
        let n = time.duration_since(std::time::UNIX_EPOCH).unwrap();
        let last_modified = format!("{}", n.as_millis());
        Metadata {
            deleted: false,
            last_modified,
            last_opened_page: 0,
            metadatamodified: false,
            modified: false,
            parent: "".into(),
            pinned: false,
            synced: false,
            typ: "DocumentType".into(),
            version: 2,
            visible_name: "".into(),
        }
    }
}

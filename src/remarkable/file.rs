use crate::{Error, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

const METADATA_EXTENSION: &str = "metadata";

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    filedata: Result<FileData>,
}

impl File {
    fn with_filedata<'a, F, T>(&'a self, f: F) -> Result<T>
    where
        F: FnOnce(&'a FileData) -> T,
    {
        self.filedata
            .as_ref()
            .map(f)
            .map_err(|_| Error::FileFailedToLoad(self.path.clone()))
    }

    pub fn visible_name(&self) -> Result<&str> {
        self.with_filedata(|fd| fd.metadata.visible_name.as_str())
    }
}

#[derive(Debug)]
pub struct FileData {
    metadata: Metadata,
}

impl FileData {
    fn load(path: impl AsRef<Path>) -> Result<FileData> {
        let metadata = Metadata::load(&path)?;
        Ok(FileData { metadata })
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    deleted: bool,
    parent: String,
    #[serde(rename = "visibleName")]
    visible_name: String,
}

impl Metadata {
    fn load(path: impl AsRef<Path>) -> Result<Metadata> {
        let md_path = path.as_ref().with_extension(METADATA_EXTENSION);
        let file = std::fs::File::open(&md_path)?;
        let metadata = serde_json::from_reader(file)?;

        Ok(metadata)
    }
}

impl File {
    pub fn load(path: PathBuf) -> Result<File> {
        let filedata = FileData::load(&path);
        Ok(File { path, filedata })
    }
}

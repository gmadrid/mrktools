use super::metadata::Metadata;
use crate::{Error, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub filedata: Result<FileData>,
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

    pub fn id(&self) -> String {
        self.path
            .file_stem()
            .map(|fs| fs.to_string_lossy().into_owned())
            .unwrap_or_else(|| "<null>".to_string())
    }

    pub fn visible_name(&self) -> Result<&str> {
        self.with_filedata(|fd| fd.metadata.visible_name.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct FileData {
    pub metadata: Metadata,
}

impl FileData {
    fn load(path: impl AsRef<Path>) -> Result<FileData> {
        let metadata = Metadata::load(&path)?;
        Ok(FileData { metadata })
    }
}

impl File {
    pub fn load(path: PathBuf) -> Result<File> {
        let filedata = FileData::load(&path);
        Ok(File { path, filedata })
    }
}

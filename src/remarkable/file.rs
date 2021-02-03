use super::metadata::Metadata;
use crate::{Error, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    pub(crate) path: PathBuf,
    pub(crate) filedata: Result<FileData>,
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
    pub(crate) metadata: Metadata,
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

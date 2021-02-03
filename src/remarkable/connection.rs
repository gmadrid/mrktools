use super::File;
use crate::{Error, Result};
use log::debug;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const DATA_DIR: &str = ".local/share/remarkable/xochitl";

pub struct Connection {
    files: Vec<File>,
    path: PathBuf,
}

impl Connection {
    pub fn new(user_home: impl AsRef<Path>) -> Result<Connection> {
        let mut conn = Connection {
            files: Default::default(),
            path: user_home.as_ref().join(DATA_DIR).to_path_buf(),
        };
        conn.sync()?;
        Ok(conn)
    }

    pub fn sync(&mut self) -> Result<()> {
        // For now, let's just load all of the file metadata in one big go.
        debug!("syncing");
        for item in read_dir(&self.path)? {
            let item = item?;
            // Load only the content files.
            if let Some(_) = item.path().extension() {
                continue;
            }
            debug!(
                "loading {}",
                item.path()
                    .file_stem()
                    .map(|fs| fs.to_string_lossy().into_owned())
                    .unwrap_or("<null>".to_owned())
            );
            let file = File::load(item.path())?;
            debug!("file loaded: {:?}", file);
            self.files.push(file);
        }
        debug!("read {} files", self.files.len());
        Ok(())
    }

    pub fn find_folder(&mut self, folder: impl AsRef<str>) -> Result<String> {
        debug!("finding '{}'", folder.as_ref());
        let found = self.files.iter().find(|f| {
            if let Ok(file_data) = f.filedata.as_ref() {
                if file_data.metadata.visible_name == folder.as_ref() {
                    return true;
                }
            }
            return false;
        });

        let result = found
            .and_then(|file| file.path.file_stem())
            .and_then(|fs| fs.to_str())
            .map(|fs| fs.to_string())
            .ok_or(Error::FolderNotFound(folder.as_ref().to_string()));
        debug!("found: {:?}", result);
        result
    }

    pub fn files(&self) -> impl Iterator<Item = &File> {
        self.files.iter()
    }
}

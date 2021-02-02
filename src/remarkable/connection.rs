use super::File;
use crate::Result;
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
        for item in read_dir(&self.path)? {
            let item = item?;
            // Load only the content files.
            if let Some(_) = item.path().extension() {
                continue;
            }
            self.files.push(File::load(item.path())?);
        }
        Ok(())
    }

    pub fn files(&self) -> impl Iterator<Item = &File> {
        self.files.iter()
    }
}

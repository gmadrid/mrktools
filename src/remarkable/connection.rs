use super::sshfs::SshFsMount;
use super::File;
use crate::{Error, Result};
use log::debug;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const DATA_DIR: &str = ".local/share/remarkable/xochitl";

pub struct Connection {
    // The connection to the Remarkable filesystem via sshfs.
    mount: SshFsMount,

    // List of files with metadata (or errors, if something couldn't be loaded)
    files: Vec<File>,

    // The full path to the mounted file system where the xochitl files live.
    path: PathBuf,
}

impl Connection {
    pub fn connect(
        user: impl AsRef<str>,
        host: impl AsRef<str>,
        mount_point: impl AsRef<str>,
    ) -> Result<Connection> {
        let mut mount = SshFsMount::new(user, host, mount_point.as_ref());
        mount.mount()?;

        let path = PathBuf::from(mount_point.as_ref())
            .join(DATA_DIR)
            .to_path_buf();

        // TODO: delay reading the files until needed.
        let mut conn = Connection {
            mount,
            files: Default::default(),
            path,
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

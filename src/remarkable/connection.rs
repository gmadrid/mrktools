use super::sshfs::SshFsMount;
use super::File;
use crate::{Error, Result};
use log::{debug, trace};
use std::cell::{Ref, RefCell};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

const DATA_DIR: &str = ".local/share/remarkable/xochitl";

pub struct Connection {
    user: String,
    host: String,

    // The connection to the Remarkable filesystem via sshfs.
    // This is never read. When dropped, it umounts the mount point.
    mount: SshFsMount,

    // List of files with metadata (or errors, if something couldn't be loaded)
    lazy_files: RefCell<Option<Vec<File>>>,

    // The full path to the mounted file system where the xochitl files live.
    path: PathBuf,
}

impl Connection {
    pub fn connect(
        user: impl AsRef<str>,
        host: impl AsRef<str>,
        mount_point: impl AsRef<str>,
    ) -> Result<Connection> {
        debug!("connecting");
        let mut mount = SshFsMount::new(&user, &host, mount_point.as_ref());
        mount.mount()?;

        let path = PathBuf::from(mount_point.as_ref()).join(DATA_DIR);

        Ok(Connection {
            user: user.as_ref().to_string(),
            host: host.as_ref().to_string(),
            mount: mount,
            lazy_files: Default::default(),
            path,
        })
    }

    pub fn mount_point(&self) -> &Path {
        self.mount.mount_point()
    }

    pub fn data_dir(&self) -> PathBuf {
        self.mount.mount_point().join(DATA_DIR)
    }

    pub fn restart(&self) -> Result<()> {
        debug!("restart()");
        Command::new("ssh")
            .arg(format!("{}@{}", self.user, self.host))
            .arg("systemctl")
            .arg("restart")
            .arg("xochitl")
            .output()?;
        trace!("restart complete");
        Ok(())
    }

    pub fn files(&self) -> Result<Ref<Vec<File>>> {
        if self.lazy_files.borrow().is_none() {
            debug!("Loading file cache.");
            let mut files = Vec::default();
            self.load_files(&mut files)?;

            self.lazy_files.replace(Some(files));
        }
        // unwrap: at this point, lazy_files should be populated.
        Ok(Ref::map(self.lazy_files.borrow(), |o| o.as_ref().unwrap()))
    }

    fn load_files(&self, files: &mut Vec<File>) -> Result<()> {
        // For now, let's just load all of the file metadata in one big go.
        debug!("loading Remarkable file metadata into local cache");
        for item in read_dir(&self.path)? {
            let item = item?;
            // Load only the metadata files.
            if !item.path().extension().map_or(false, |f| f == "metadata") {
                continue;
            }
            trace!(
                "loading {}",
                item.path()
                    .file_stem()
                    .map(|fs| fs.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "<null>".to_owned())
            );
            let file = File::load(item.path())?;
            trace!("file loaded: {:?}", file);
            files.push(file);
        }
        debug!("read {} files", files.len());
        Ok(())
    }

    pub fn find_folder(&self, folder: impl AsRef<str>) -> Result<String> {
        debug!("finding '{}'", folder.as_ref());
        let file_ref = self.files()?;
        let found = file_ref.iter().find(|f| {
            if let Ok(file_data) = &f.filedata {
                if file_data.metadata.visible_name == folder.as_ref()
                    && file_data.metadata.typ == "CollectionType"
                {
                    return true;
                }
            }
            false
        });

        let result = found
            .and_then(|file| file.path.file_stem())
            .and_then(|fs| fs.to_str())
            .map(|fs| fs.to_string())
            .ok_or_else(|| Error::FolderNotFound(folder.as_ref().to_string()));
        debug!("found: {:?}", result);
        result
    }
}

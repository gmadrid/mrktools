use crate::{Error, Result};
use log::{debug, error};
use std::path::{Path, PathBuf};
use std::process::Command;

const SSHFS_COMMAND: &str = "sshfs";

pub struct SshFsMount {
    host: String,
    user: String,
    path: PathBuf,
    mounted: bool,
}

impl SshFsMount {
    pub fn new(
        user: impl AsRef<str>,
        host: impl AsRef<str>,
        mount_point: impl AsRef<Path>,
    ) -> Self {
        SshFsMount {
            host: host.as_ref().to_string(),
            user: user.as_ref().to_string(),
            path: mount_point.as_ref().to_path_buf(),
            mounted: false,
        }
    }

    pub fn mount(&mut self) -> Result<()> {
        mount_sshdir(&self.user, &self.host, &self.path)?;
        self.mounted = true;
        Ok(())
    }

    pub fn unmount(&mut self) -> Result<()> {
        if self.mounted {
            debug!("unmounting");
            self.mounted = false;
            umount_sshdir(&self.path)
        } else {
            Ok(())
        }
    }
}

impl Drop for SshFsMount {
    fn drop(&mut self) {
        match self.unmount() {
            Err(err) => error!("unmount error: {}", err),
            _ => {}
        }
    }
}

fn mount_sshdir(
    user: impl AsRef<str>,
    host: impl AsRef<str>,
    mount: impl AsRef<Path>,
) -> Result<()> {
    let mount_path = mount.as_ref();

    if mount_path.exists() {
        return Err(Error::MountPointExistsErr(mount_path.to_path_buf()));
    }

    debug!("creating mount point at {}", mount_path.to_string_lossy());
    std::fs::create_dir(&mount_path)?;

    debug!("mounting {}@{}", user.as_ref(), host.as_ref());
    Command::new(SSHFS_COMMAND)
        .arg(&format!("{}@{}:", user.as_ref(), host.as_ref()))
        .arg(&mount_path)
        .output()?;
    debug!("mounted.");
    // TODO: should we check this output.

    // TODO: do we have to own this?
    Ok(())
}

fn umount_sshdir(path: impl AsRef<Path>) -> Result<()> {
    Command::new("umount").arg(path.as_ref()).output()?;
    std::fs::remove_dir(path)?;
    Ok(())
}

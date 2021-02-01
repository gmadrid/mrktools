use crate::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

const SSHFS_COMMAND: &str = "sshfs";

pub fn mount_sshdir(
    user: impl AsRef<str>,
    host: impl AsRef<str>,
    mount: impl AsRef<Path>,
) -> Result<PathBuf> {
    let mount_path = mount.as_ref();

    if mount_path.exists() {
        return Err(Error::MountPointExistsErr(mount_path.to_path_buf()));
    }

    std::fs::create_dir(&mount_path)?;

    Command::new(SSHFS_COMMAND)
        //.stdin(Stdio::piped())
        .arg(&format!("{}@{}:", user.as_ref(), host.as_ref()))
        .arg(&mount_path)
        .output()?;

    // TODO: should we check this output.

    // TODO: do we have to own this?
    Ok(mount_path.to_path_buf())
}

pub fn umount_sshdir(path: impl AsRef<Path>) -> Result<()> {
    Command::new("umount").arg(path.as_ref()).output()?;
    std::fs::remove_dir(path)?;
    Ok(())
}

/*

  1. mount ssh dir



  X. umount sshdir
*/

mod result;
mod sshfs;

use result::{Error, Result};
use sshfs::{mount_sshdir, umount_sshdir};
use std::path::Path;
use std::process::Command;

// TODO: all of these should be command line args
const MOUNT_POINT: &str = "/tmp/remarkable_mount";
const REMARKABLE_HOST: &str = "192.168.86.31";
//const REMARKABLE_PASSWORD: &str = "9aaVMBIzcD";
const REMARKABLE_USER: &str = "root";

const DATA_DIR: &str = ".local/share/remarkable/xochitl";

fn with_sshdir<F>(f: F) -> Result<()>
where
    F: FnOnce(&Path) -> Result<()>,
{
    let mount_path = mount_sshdir(REMARKABLE_USER, REMARKABLE_HOST, MOUNT_POINT)?;

    let result = f(&mount_path);

    let res2 = umount_sshdir(&mount_path);

    // Return any error with umount only if the passed function failed.
    if result.is_ok() && res2.is_err() {
        res2
    } else {
        result
    }
}

fn main() -> Result<()> {
    with_sshdir(|mount| {
        let path = mount.join(DATA_DIR);

        println!("Reading from: '{:?}'", path);

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();

            println!("{}", path.to_string_lossy());
        }

        Ok(())
    })
}

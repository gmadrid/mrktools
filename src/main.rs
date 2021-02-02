mod remarkable;
mod result;
mod sshfs;
// mod tester;

use mrktools::{Error, Result};
use remarkable::Connection;
use sshfs::{mount_sshdir, umount_sshdir};
use std::path::Path;

// TODO: all of these should be command line args
const MOUNT_POINT: &str = "/tmp/remarkable_mount";
const REMARKABLE_HOST: &str = "192.168.86.31";
//const REMARKABLE_PASSWORD: &str = "9aaVMBIzcD";
const REMARKABLE_USER: &str = "root";

// fn with_sshdir<F>(f: F) -> Result<()>
// where
//     F: FnOnce(&Path) -> Result<()>,
// {
//     let mount_path = mount_sshdir(REMARKABLE_USER, REMARKABLE_HOST, MOUNT_POINT)?;
//
//     let result = f(&mount_path);
//
//     let res2 = umount_sshdir(&mount_path);
//
//     // Return any error with umount only if the passed function failed.
//     if result.is_ok() && res2.is_err() {
//         res2
//     } else {
//         result
//     }
// }

fn main() -> Result<()> {
    mrktools::i2pdf("./test.jpg")?;
    // with_sshdir(|mount| {
    //     let connection = Connection::new(mount)?;
    //     for f in connection.files() {
    //         println!("{}", f.visible_name()?);
    //     }
    //     Ok(())
    // })
    Ok(())
}

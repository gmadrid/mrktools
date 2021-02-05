use crate::remarkable::Connection;
use crate::Result;
use argh::FromArgs;
use log::{info, trace};
use std::path::{Path, PathBuf};

#[derive(FromArgs, Debug)]
/// list files on Remarkable
#[argh(subcommand, name = "copy")]
pub struct CopierArgs {
    /// the source directory
    #[argh(positional)]
    src: PathBuf,

    /// the destination directory
    #[argh(positional)]
    dest: Option<PathBuf>,
}

pub fn copy(conn: &Connection, args: CopierArgs) -> Result<()> {
    // There will always be a source.
    let src = args.src.as_path();

    // If no destination is provided, then defaunt to the connection mount point.
    let data_dir = conn.data_dir();
    let dst = args
        .dest
        .as_ref()
        .map(|d| d.as_path())
        .unwrap_or(data_dir.as_path());

    copy_fn(src, dst)
}

pub(crate) fn copy_fn(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    info!("Copying from {:?} to {:?}", src.as_ref(), dst.as_ref());

    let mut files = Vec::default();

    collect_files_from_dir(&src, &mut files)?;

    for path in files {
        let dest_filename = dst.as_ref().join(path.strip_prefix(&src)?);

        trace!("copying {:?} ==> {:?}", path, dest_filename);

        if let Some(dest_parent) = dest_filename.parent() {
            if !dest_parent.exists() {
                std::fs::create_dir_all(dest_parent)?;
            }
        }

        std::fs::copy(path, dest_filename)?;
    }

    Ok(())
}

fn collect_files_from_dir(dir: impl AsRef<Path>, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry_ in walkdir::WalkDir::new(dir).same_file_system(true) {
        let entry = entry_?;
        if entry.metadata()?.is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    Ok(())
}

use crate::remarkable::Connection;
use crate::Result;
use argh::FromArgs;
use log::{info, trace};
use std::path::{Path, PathBuf};

#[derive(FromArgs, Debug)]
/// list files on Remarkable
#[argh(subcommand, name = "copy")]
pub struct CopierArgs {
    /// if present, restart the Remarkable app before quitting.
    #[argh(switch, short = 'r')]
    restart: bool,

    /// the source directory
    #[argh(positional)]
    src: PathBuf,

    /// the destination directory
    #[argh(positional)]
    dest: Option<PathBuf>,
}

pub fn copy(conn: &Connection, args: CopierArgs) -> Result<()> {
    copy_fn(conn, args.src, args.dest)?;

    if args.restart {
        conn.restart()?;
    }
    Ok(())
}

pub fn copy_fn(
    conn: &Connection,
    src: impl AsRef<Path>,
    dst: Option<impl AsRef<Path>>,
) -> Result<()> {
    // TODO: can we avoid this call unless necessary?
    let data_dir = conn.data_dir();
    let dst = dst
        .as_ref()
        .map(|d| d.as_ref())
        .unwrap_or_else(|| data_dir.as_path());

    copy_helper(src, dst)?;
    Ok(())
}

fn copy_helper(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
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

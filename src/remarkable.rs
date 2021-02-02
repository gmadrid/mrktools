// mod connection;
mod content;
// mod file;
mod metadata;

use crate::Result;
// pub use connection::Connection;
pub use content::Content;
// use file::File;
pub use metadata::Metadata;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

pub const METADATA_EXTENSION: &str = "metadata";

pub fn create_bare_fs(uuid: impl AsRef<str>, root: impl AsRef<Path>) -> Result<PathBuf> {
    let base = root.as_ref().join(uuid.as_ref());

    create_dir(&base)?;
    create_dir(&base.with_extension("highlights"))?;
    create_dir(&base.with_extension("textconvertion"))?;
    create_dir(&base.with_extension("thumbnails"))?;

    Ok(base)
}

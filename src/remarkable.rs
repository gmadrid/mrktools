mod connection;
pub use connection::Connection;

mod content;
pub use content::Content;

mod file;
pub use file::File;

mod metadata;
pub use metadata::Metadata;

mod sshfs;

use crate::Result;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

pub const METADATA_EXTENSION: &str = "metadata";

/// Creates all of the directories (but none of the files) required for a new Remarkable item.
/// The file stem for these directories will be based on the supplied uuid. All directories will
/// be created in the `root` directory.
///
/// The file structure is all reverse-engineered by third parties. Here is what is known
/// at this time.
///
/// extension           | type           | create  | description
/// --------------------|----------------|---------|------------
/// ___<none>___        | __directory__  | __Yes__ | annotations will be stored here
/// .cache              | directory      | No      | not sure but empty initially
/// .content            | file           | No      | information including uuids for each page
/// __.highlights__     | __directory__  | __Yes__ | not sure but empty initially
/// .metadata           | file           | No      | name, dates, type
/// .pagedata           | file           | No      | list of templates, one per page
/// .pdf                | file           | No      | the actual PDF uploaded
/// __.textconversion__ | __directory__  | __Yes__ | for converted annotations; empty initially
/// __.thumbnails__     | __directory__  | __Yes__ | 362x512 pixel jpeg, one per page in document numbered 0, 1, …, n-1
///
pub fn create_bare_fs(uuid: impl AsRef<str>, root: impl AsRef<Path>) -> Result<PathBuf> {
    let base = root.as_ref().join(uuid.as_ref());

    create_dir(&base)?;
    create_dir(&base.with_extension("highlights"))?;
    create_dir(&base.with_extension("textconvertion"))?;
    create_dir(&base.with_extension("thumbnails"))?;

    Ok(base)
}

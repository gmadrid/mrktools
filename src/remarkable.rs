mod connection;
mod content;
mod file;

use crate::Result;
pub use connection::Connection;
pub use content::Content;
use file::File;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

fn generate_uuid() -> String {
    "georgeuuid".to_string()
}

pub fn create_bare_fs(root: impl AsRef<Path>) -> Result<PathBuf> {
    let uuid = generate_uuid();

    let base = root.as_ref().join(uuid);

    create_dir(&base)?;
    create_dir(&base.with_extension("highlights"))?;
    create_dir(&base.with_extension("textconvertion"))?;
    create_dir(&base.with_extension("thumbnails"))?;

    Ok(base)
}

/*

| directory	| annotations will be stored here
.cache	       | directory	| not sure but empty initially
.content       | file	      | information including uuids for each page
.highlights    | directory  | not sure but empty initially
.metadata      | file	      | name, dates, type
.pagedata      | file       | list of templates, one per page
.pdf           | file       | the actual PDF uploaded
.textconversion| directory  | for converted annotations; empty initially
.thumbnails    | directory  | 362x512 pixel jpeg, one per page in document numbered 0, 1, …, n-1


*/

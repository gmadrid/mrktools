use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The mount point, '{0}', already exists.")]
    MountPointExistsErr(PathBuf),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

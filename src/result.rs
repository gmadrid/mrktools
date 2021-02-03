use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Alpha value, {0}, out of range, [0..100]")]
    AlphaRangeError(u8),

    #[error("The file at {0} failed to load")]
    FileFailedToLoad(PathBuf),

    #[error("The mount point, '{0}', already exists.")]
    MountPointExistsErr(PathBuf),

    #[error("ImageError: {0}")]
    ImageError(#[from] printpdf::image::ImageError),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("PrintPdfError: {0}")]
    PrintPdfError(#[from] printpdf::Error),

    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

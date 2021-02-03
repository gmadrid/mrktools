use argh::FromArgs;
use mrktools::{i2pdf, Error, Result};
use std::ffi::OsString;
use std::path::PathBuf;

/// Create a PDF file with thumbnails from an image for the Remarkable.
#[derive(FromArgs)]
struct Opt {
    /// alpha value to be multiplied by the image, range [0-100].
    #[argh(option, short = 'a', default = "100")]
    alpha: u8,

    /// convert pdf to grayscale
    #[argh(switch, short = 'g')]
    to_gray: bool,

    /// file names to convert to Remarkable FDF files
    #[argh(positional)]
    file_names: Vec<String>,
}

impl Opt {
    fn validate(self) -> Result<Opt> {
        if self.alpha > 100 {
            return Err(Error::AlphaRangeError(self.alpha));
        }
        Ok(self)
    }
}

// TODO: all of these should be command line args
// const MOUNT_POINT: &str = "/tmp/remarkable_mount";
// const REMARKABLE_HOST: &str = "192.168.86.31";
// //const REMARKABLE_PASSWORD: &str = "9aaVMBIzcD";
// const REMARKABLE_USER: &str = "root";

fn main() {
    let opt = argh::from_env::<Opt>().validate();
    if let Err(err) = opt {
        eprintln!("Error: {}", err);
        return;
    }

    let opt = opt.unwrap();

    let should_print = opt.file_names.len() > 1;
    for file in opt.file_names {
        if should_print {
            let base_fn = PathBuf::from(&file)
                .file_name()
                .map(|f| OsString::from(f))
                .unwrap_or(OsString::from(""));
            println!("Processing: {}", base_fn.to_string_lossy());
        }
        match i2pdf(file, opt.to_gray, opt.alpha) {
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
}

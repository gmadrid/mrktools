use argh::FromArgs;
use log::{debug, error, info};
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

    /// if set, restart the Remarkable app when done
    #[argh(switch, short = 'r')]
    restart: bool,
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
const MOUNT_POINT: &str = "/tmp/remarkable_mount";
const REMARKABLE_HOST: &str = "192.168.86.31";
const REMARKABLE_USER: &str = "root";

fn process_opts(opt: Opt) -> Result<()> {
    let mut conn = mrktools::Connection::connect(REMARKABLE_USER, REMARKABLE_HOST, MOUNT_POINT)
        .expect("conn failed");

    let folder_uuid = conn.find_folder("To Draw").expect("folder not found");
    debug!("found {}", folder_uuid);

    let should_print = opt.file_names.len() > 1;
    for file in opt.file_names {
        if should_print {
            let base_fn = PathBuf::from(&file)
                .file_name()
                .map(OsString::from)
                .unwrap_or_else(|| OsString::from(""));
            info!("Processing: {}", base_fn.to_string_lossy());
        }
        if let Err(err) = i2pdf(file, opt.to_gray, opt.alpha) {
            error!("{}", err);
        }
    }

    if opt.restart {
        if let Err(err) = conn.restart() {
            error!("{}", err);
        }
    }
    Ok(())
}

fn main() {
    pretty_env_logger::init();

    match argh::from_env::<Opt>().validate() {
        Err(err) => error!("{}", err),
        Ok(opt) => {
            if let Err(err) = process_opts(opt) {
                error!("{}", err)
            }
        }
    }
}

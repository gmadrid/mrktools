use argh::FromArgs;
use log::{error, info};
use mrktools::{i2pdf, Error, Result};
use std::ffi::OsString;
use std::path::PathBuf;

const MOUNT_POINT_DEFAULT: &str = "/tmp/remarkable_mount";
const REMARKABLE_HOST_DEFAULT: &str = "192.168.86.31";
const REMARKABLE_USER_DEFAULT: &str = "root";

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

    /// ip address or hostname of the Remarkable device
    #[argh(option, short = 'h', default = "REMARKABLE_HOST_DEFAULT.to_string()")]
    host: String,

    /// username of the ssh user on the Remarkable device
    #[argh(option, short = 'u', default = "REMARKABLE_USER_DEFAULT.to_string()")]
    user: String,

    /// directory onto which to mount the Remarkable fs.
    /// Should not exist, and it will be deleted on a normal exit.
    #[argh(option, short = 'm', default = "MOUNT_POINT_DEFAULT.to_string()")]
    mount_point: String,

    /// if set, restart the Remarkable app when done
    #[argh(switch, short = 'r')]
    restart: bool,

    /// if present, generated files will be put in the specified folder on the Remarkable
    #[argh(option, short = 'p')]
    parent: Option<String>,
}

impl Opt {
    fn validate(self) -> Result<Opt> {
        if self.alpha > 100 {
            return Err(Error::AlphaRangeError(self.alpha));
        }
        Ok(self)
    }
}

fn process_opts(opt: Opt) -> Result<()> {
    let mut conn = mrktools::Connection::connect(opt.user, opt.host, opt.mount_point)?;

    let should_print = opt.file_names.len() > 1;
    for file in opt.file_names {
        if should_print {
            let base_fn = PathBuf::from(&file)
                .file_name()
                .map(OsString::from)
                .unwrap_or_else(|| OsString::from(""));
            info!("Processing: {}", base_fn.to_string_lossy());
        }

        let parent_id = opt
            .parent
            .as_ref()
            .map(|p| conn.find_folder(p))
            .transpose()?;
        if let Err(err) = i2pdf(file, opt.to_gray, opt.alpha, parent_id) {
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

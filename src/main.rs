use mrktools::{i2pdf, Error, Result};
use std::ffi::OsString;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mrktools",
    about = "A collection of tools for working with Remarkable files."
)]
struct Opt {
    #[structopt(name = "FILE")]
    file_names: Vec<String>,

    #[structopt(short = "a", long = "alpha", default_value = "100")]
    alpha: u8,

    #[structopt(short = "g", long = "gray")]
    to_gray: bool,
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
    let opt = Opt::from_args().validate();
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

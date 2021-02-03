use mrktools::{i2pdf, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mrktools",
    about = "A collection of tools for working with Remarkable files."
)]
struct Opt {
    #[structopt(name = "FILE")]
    file_names: Vec<String>,
}

// TODO: all of these should be command line args
// const MOUNT_POINT: &str = "/tmp/remarkable_mount";
// const REMARKABLE_HOST: &str = "192.168.86.31";
// //const REMARKABLE_PASSWORD: &str = "9aaVMBIzcD";
// const REMARKABLE_USER: &str = "root";

fn main() -> Result<()> {
    let opt = Opt::from_args();
    for file in opt.file_names {
        i2pdf(file)?;
    }
    Ok(())
}

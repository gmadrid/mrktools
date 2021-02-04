use argh::FromArgs;
use log::{error, info};
use mrktools::{i2pdf, Connection, File, Result};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

const DEFAULT_DEST_DIR: &str = "./rem";
const MOUNT_POINT_DEFAULT: &str = "/tmp/remarkable_mount";
const REMARKABLE_HOST_DEFAULT: &str = "192.168.86.31";
const REMARKABLE_USER_DEFAULT: &str = "root";

#[derive(FromArgs, Debug)]
/// Top-level commands
struct Commands {
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

    #[argh(subcommand)]
    nested: CommandsEnum,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum CommandsEnum {
    IPdf(IPdfArgs),
    Ls(LsArgs),
    Restart(RestartArgs),
}

#[derive(FromArgs, Debug)]
/// convert images to Remarkable-ready pdfs with a main thumbnail
#[argh(subcommand, name = "ipdf")]
struct IPdfArgs {
    /// alpha value to be multiplied by the image, range [0-100].
    #[argh(option, short = 'a', default = "100")]
    alpha: u8,

    /// convert pdf to grayscale
    #[argh(switch, short = 'g')]
    to_gray: bool,

    /// file names to convert to Remarkable FDF files
    #[argh(positional)]
    file_names: Vec<String>,

    /// if present, generated files will be put in the specified folder on the Remarkable
    #[argh(option, short = 'p')]
    parent: Option<String>,

    /// directory for output files
    #[argh(option, short = 'o', default = "DEFAULT_DEST_DIR.to_string()")]
    dest_dir: String,
}

#[derive(FromArgs, Debug)]
/// list "files" on Remarkable
#[argh(subcommand, name = "ls")]
struct LsArgs {}

#[derive(FromArgs, Debug)]
/// restart the Remarkable
#[argh(subcommand, name = "restart")]
struct RestartArgs {}

fn lsf(conn: &mut mrktools::Connection) -> Result<()> {
    let files = conn.files()?;

    //let mut file_set: HashSet<&File> = files.iter().collect();
    let mut file_hash: HashMap<String, Vec<&File>> = Default::default();
    for file in files.iter() {
        if let Ok(file_data) = file.filedata.as_ref() {
            let v = file_hash
                .entry(file_data.metadata.parent.clone())
                .or_default();
            v.push(file);
        }
    }

    ls_helper(&file_hash, "", "");

    Ok(())
}

fn ls_helper(file_hash: &HashMap<String, Vec<&File>>, parent: &str, prefix: &str) {
    if let Some(curr_vec) = file_hash.get(parent) {
        let mut sorted_vec = curr_vec.iter().collect::<Vec<_>>();
        // unwrap: just rewrite this.
        sorted_vec.sort_by(|f1, f2| f1.visible_name().unwrap().cmp(f2.visible_name().unwrap()));
        for file in sorted_vec {
            if let Ok(file_data) = file.filedata.as_ref() {
                if file_data.metadata.typ == "CollectionType" {
                    println!("{}{}/", prefix, file_data.metadata.visible_name);
                    ls_helper(file_hash, &file.id(), &format!("   {}", prefix));
                } else {
                    println!("{}{}", prefix, file_data.metadata.visible_name);
                }
            }
        }
    } else {
        eprintln!("EXPECTED TO FIND: {}", parent);
    }
}

fn ipdf(conn: &mut Connection, opt: IPdfArgs) -> Result<()> {
    let should_print = opt.file_names.len() > 1;
    Ok(for file in opt.file_names {
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
        if let Err(err) = i2pdf(file, opt.to_gray, opt.alpha, parent_id, &opt.dest_dir) {
            error!("{}", err);
        }
    })
}

fn ls(conn: &mut Connection, _: LsArgs) -> Result<()> {
    lsf(conn)
}

fn restart(conn: &Connection, _: RestartArgs) -> Result<()> {
    conn.restart()
}

fn with_connection<F>(
    user: impl AsRef<str>,
    host: impl AsRef<str>,
    mount_point: impl AsRef<str>,
    f: F,
) -> Result<()>
where
    // TODO: get this "mut" out of here
    F: FnOnce(&mut Connection) -> Result<()>,
{
    let mut conn = Connection::connect(user, host, mount_point)?;
    f(&mut conn)
}

fn main() {
    pretty_env_logger::init();

    let args = argh::from_env::<Commands>();
    if let Err(err) = match args.nested {
        CommandsEnum::IPdf(a) => {
            with_connection(&args.user, &args.host, &args.mount_point, |conn| {
                ipdf(conn, a)
            })
        }
        CommandsEnum::Ls(a) => with_connection(&args.user, &args.host, &args.mount_point, |conn| {
            ls(conn, a)
        }),
        CommandsEnum::Restart(a) => {
            with_connection(&args.user, &args.host, &args.mount_point, |conn| {
                restart(conn, a)
            })
        }
    } {
        error!("{}", err);
    }
}

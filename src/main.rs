use argh::FromArgs;
use log::error;
use mrktools::subcommands::{ipdf, ls, restart};
use mrktools::{Connection, Result};

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
    IPdf(ipdf::IPdfArgs),
    Ls(ls::LsArgs),
    Restart(restart::RestartArgs),
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
                ipdf::ipdf(conn, a)
            })
        }
        CommandsEnum::Ls(a) => with_connection(&args.user, &args.host, &args.mount_point, |conn| {
            ls::ls(conn, a)
        }),
        CommandsEnum::Restart(a) => {
            with_connection(&args.user, &args.host, &args.mount_point, |conn| {
                restart::restart(conn, a)
            })
        }
    } {
        error!("{}", err);
        eprintln!("Error: {}", err);
    }
}

use crate::remarkable::Connection;
use crate::Result;
use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// restart the Remarkable
#[argh(subcommand, name = "restart")]
pub struct RestartArgs {}

pub fn restart(conn: &Connection, _: RestartArgs) -> Result<()> {
    conn.restart()
}

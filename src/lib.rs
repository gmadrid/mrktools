mod imgtools;
mod ipdf;
mod remarkable;
mod result;

pub use ipdf::i2pdf;
pub use remarkable::{Connection, File};
pub use result::{Error, Result};
use uuid::Uuid;

fn new_uuid() -> String {
    let uu = Uuid::new_v4();
    uu.to_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_string()
}

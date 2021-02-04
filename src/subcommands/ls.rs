use crate::remarkable::Connection;
use crate::File;
use crate::Result;
use argh::FromArgs;
use std::collections::HashMap;

#[derive(FromArgs, Debug)]
/// list files on Remarkable
#[argh(subcommand, name = "ls")]
pub struct LsArgs {}

pub fn ls(conn: &mut Connection, _: LsArgs) -> Result<()> {
    ls_func(conn)
}

fn ls_func(conn: &mut Connection) -> Result<()> {
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

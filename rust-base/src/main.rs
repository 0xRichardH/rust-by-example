mod error;
mod prelude;
mod utils;

use std::fs::read_dir;

use crate::prelude::*;

fn main() -> Result<()> {
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }

    Ok(())
}

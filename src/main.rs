//! Jeremy Chone's Rust Quick Start Template
//! 
//! See: [https://www.youtube.com/watch?v=oxx7MmN4Ib0](https://www.youtube.com/watch?v=oxx7MmN4Ib0)

//#![allow(unused)]  // For beginning only.

mod error;
mod prelude;
mod utils;

use crate::prelude::*;
use std::fs::read_dir;


fn main() -> Result<()> {
    println!("Hello, world!");
    println!();

    for entry in read_dir("./")
        ?.filter_map(|e| e.ok()) {

            // Convert directory entry to string
            let entry: String = Wrapper(&entry).try_into()?;

            println!("{entry}");
    }

    println!();

    Ok(())
}

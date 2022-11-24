//! Jeremy Chone's Rust Quick Start Template
//! 
//! See: [https://www.youtube.com/watch?v=oxx7MmN4Ib0](https://www.youtube.com/watch?v=oxx7MmN4Ib0)

#![allow(unused)]  // For beginning only.

mod error;
mod prelude;
mod utils;

use crate::prelude::*;


fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

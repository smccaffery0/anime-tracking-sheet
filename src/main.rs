//! Entry point for 'anime-tracking-sheet'
#![allow(clippy::cargo_common_metadata)]
use crate::table::create_table;
mod table;
use std::io;

fn main() -> io::Result<()> {
    let _ = create_table();
    Ok(())
}

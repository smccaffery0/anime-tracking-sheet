//! Entry point for 'anime-tracking-sheet'
#![allow(clippy::cargo_common_metadata)]
use crate::table::create_table;
mod table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_table()?;
    Ok(())
}

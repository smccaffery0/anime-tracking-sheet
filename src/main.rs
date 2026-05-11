//! Entry point for 'anime-tracking-sheet'
#![allow(clippy::cargo_common_metadata)]
use crate::table::create_table;
mod table;

fn main() {
    create_table();
}

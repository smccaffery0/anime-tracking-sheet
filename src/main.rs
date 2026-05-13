//! Entry point for 'anime-tracking-sheet'
#![allow(clippy::cargo_common_metadata)]
use std::{io, process::exit};

use crate::table::{create_table, print_table, update_table};
mod table;

// Display menu for user
fn menu() {
    println!("[1] => Add new anime");
    println!("[2] => View Anime Table");
    println!("[3] => Update The Table");
    println!("[4] => Quit");
}

// Get option input for user
fn main() -> Result<(), Box<dyn std::error::Error>> {
    menu();
    println!("Select an option: ");
    let mut option_selected = String::new();
    let _ = io::stdin().read_line(&mut option_selected);
    let option_selected = option_selected.trim();

    // match based on user input
    match option_selected {
        "1" => create_table()?,
        "2" => print_table()?,
        "3" => update_table(),
        "4" => exit(0),
        _ => println!("Not an option!"),
    }
    Ok(())
}

use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use core::error;
use csv::{self};
use serde_derive::{Deserialize, Serialize};
use std::{
    io::{self, stdin},
    process::{Command, exit},
};

// Table data input
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeSheet {
    pub anime_count: String,
    pub anime_title: String,
    pub episode_count: String,
    pub user_rating: String,
    pub user_progress: String,
}

// Take user input for filling out the spreadsheet
pub fn fill_table() -> AnimeSheet {
    println!("Enter anime count: ");
    let mut anime_count = String::new();
    let _ = stdin().read_line(&mut anime_count);
    let anime_count = anime_count.trim().to_string();

    println!("Enter anime title: ");
    let mut anime_title = String::new();
    let _ = stdin().read_line(&mut anime_title);
    let anime_title = anime_title.trim().to_string();

    println!("Enter episode count: ");
    let mut episode_count = String::new();
    let _ = stdin().read_line(&mut episode_count);
    let episode_count = episode_count.trim().to_string();

    println!("\nEnter your rating: ");
    let mut user_rating = String::new();
    let _ = stdin().read_line(&mut user_rating);
    let user_rating = user_rating.trim().to_string();

    //TODO! Create an update fnc for the table
    println!("\nEnter your rating: ");
    let mut user_progress = String::new();
    let _ = io::stdin().read_line(&mut user_progress);
    let user_progress = user_progress.trim().to_string();

    //Return the struct
    AnimeSheet {
        anime_count,
        anime_title,
        episode_count,
        user_rating,
        user_progress,
    }
}

// Define the Table
pub fn create_table() -> Result<(), Box<dyn error::Error>> {
    println!("Do want to enter a title: ");
    let mut add_more = String::new();
    let _ = io::stdin().read_line(&mut add_more);
    let add_more = add_more.trim();
    if add_more == "y" {
        loop {
            let sheet = fill_table();
            let mut anime_table = Table::new();

            //Write and save to file
            let file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("anime.csv")?; // this only work if you are in src directory

            let mut wtr = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);

            wtr.serialize(&sheet)?;
            wtr.flush()?;

            // defines table properties
            anime_table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
                .set_header(vec![
                    "Count",
                    "Title",
                    "Episodes",
                    "Rating",
                    "User Progress",
                ]);

            let rows = vec![
                &sheet.anime_count,
                &sheet.anime_title,
                &sheet.episode_count,
                &sheet.user_rating,
                &sheet.user_progress,
            ];
            anime_table.add_row(rows);
            println!("Do want to enter another title: ");
            let mut add_again = String::new();
            let _ = io::stdin().read_line(&mut add_again);
            let add_again = add_again.trim();
            if add_again == "y" {
                fill_table();
            } else {
                exit(0)
            }
        }
    } else {
        exit(0);
    }
}

pub fn update_table() {
    println!("Would you like to update any current entries in the table: ");
    let mut update_entries = String::new();
    io::stdin().read_line(&mut update_entries);
    let update_entries = update_entries.trim();
    if update_entries == "y" {
        std::process::Command::new("xdg-open")
            .arg("anime.csv")
            .spawn();
    } else {
        exit(0);
    }
}

pub fn draw_table() -> Result<(), Box<dyn std::error::Error>> {
    //TODO Read csv and  print it out to terminal
    Ok(())
}

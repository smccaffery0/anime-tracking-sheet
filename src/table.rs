use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use core::error;
use csv::{self};
use serde_derive::{Deserialize, Serialize};
use std::io::{self};

// Table data input
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeSheet {
    pub anime_count: String,
    pub anime_title: String,
    pub episode_count: String,
    pub user_rating: String,
    //pub anime_status: String,
}

// Take user input for filling out the spreadsheet
pub fn fill_table() -> AnimeSheet {
    println!("Enter anime count: ");
    let mut anime_count = String::new();
    let _ = io::stdin().read_line(&mut anime_count);
    let anime_count = anime_count.trim().to_string();

    println!("Enter anime title: ");
    let mut anime_title = String::new();
    let _ = io::stdin().read_line(&mut anime_title);
    let anime_title = anime_title.trim().to_string();

    println!("Enter episode count: ");
    let mut episode_count = String::new();
    let _ = io::stdin().read_line(&mut episode_count);
    let episode_count = episode_count.trim().to_string();

    println!("\nEnter your rating: ");
    let mut user_rating = String::new();
    let _ = io::stdin().read_line(&mut user_rating);
    let user_rating = user_rating.trim().to_string();

    //TODO! //Create another column for progress/status
    //println!("\nEnter your rating: ");
    //let mut user_rating = String::new();
    //let _ = io::stdin().read_line(&mut user_rating);
    //let user_rating = user_rating.trim().to_string();

    //Return the struct
    AnimeSheet {
        anime_count,
        anime_title,
        episode_count,
        user_rating,
        //anime_status,
    }
}

// Define the Table
pub fn create_table() -> Result<(), Box<dyn error::Error>> {
    let sheet = fill_table();

    let mut anime_table = Table::new();

    //Write and save to file
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("anime.csv")?;

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
        .set_header(vec!["Count", "Title", "Episodes", "Rating"]);

    let rows = vec![
        &sheet.anime_count,
        &sheet.anime_title,
        &sheet.episode_count,
        &sheet.user_rating,
        //&sheet.anime_status,
    ];
    anime_table.add_row(rows);
    Ok(())
}

pub fn draw_table() -> Result<(), Box<dyn std::error::Error>> {
    //TODO! //read and print out table
    Ok(())
}

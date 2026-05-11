use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use serde_derive::Deserialize;
use std::io::{self};

// Table data input
#[derive(Deserialize, Debug)]
pub struct AnimeSheet {
    pub anime_count: String,
    pub anime_title: String,
    pub episode_count: String,
    pub user_rating: String,
}

pub fn fill_table() -> AnimeSheet {
    println!("Enter anime count: ");
    let mut anime_count = String::new();
    io::stdin().read_line(&mut anime_count);
    let anime_count = anime_count.trim().to_string();

    println!("Enter anime title: ");
    let mut anime_title = String::new();
    io::stdin().read_line(&mut anime_title);
    let anime_title = anime_title.trim().to_string();

    println!("Enter episode count: ");
    let mut episode_count = String::new();
    io::stdin().read_line(&mut episode_count);
    let episode_count = episode_count.trim().to_string();

    println!("Enter your rating: ");
    let mut user_rating = String::new();
    io::stdin().read_line(&mut user_rating);
    let user_rating = user_rating.trim().to_string();

    //Return the struct
    AnimeSheet {
        anime_count,
        anime_title,
        episode_count,
        user_rating,
    }
}

// Define the Table
pub fn create_table() {
    let sheet = fill_table();
    let mut anime_table = Table::new();

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
    ];
    anime_table.add_row(rows);
    // Print the table
    println!("{anime_table}");
}

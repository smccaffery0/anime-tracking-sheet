use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct AnimeSheet {
    anime_count: u32,
    anime_title: String,
    episode_count: u32,
    user_rating: f32,
}

// Define the Table
pub fn create_table() -> Result<(), Box<dyn Error>> {
    let mut anime_table = Table::new();

    anime_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .set_header(vec!["Count", "Title", "Episodes", "Rating"]);
    //.add_row(vec![animesheet]);
    println!("{anime_table}");
    Ok(())
}

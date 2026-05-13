use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use core::error;
use csv::{self, Reader, StringRecord};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, stdin},
    process::{Command, exit},
};

// Table data input
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeSheet {
    anime_count: String,
    anime_title: String,
    episode_count: String,
    user_rating: String,
    user_progress: String,
}

// Anime status possible states
#[derive(Debug)]
pub enum AnimeStatus {
    Completed,
    InProgress,
    Dropped,
    PlanOnWatching,
}

impl AnimeStatus {
    fn _check(&self) {
        match self {
            AnimeStatus::Completed => {
                println!("Anime is completed");
            }

            AnimeStatus::InProgress => {
                println!("Anime is in progress");
            }

            AnimeStatus::Dropped => {
                println!("Anime was dropped");
            }

            AnimeStatus::PlanOnWatching => {
                println!("Plan on watching");
            }
        }
    }
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
    println!(
        "\n Enter => [cmp] for complete \n Enter => [ip] in progress \n Enter => [drp] dropped "
    );
    println!("\nEnter your status: ");
    let mut user_progress = String::new();
    let _ = io::stdin().read_line(&mut user_progress);
    let user_progress = user_progress.trim().to_string();

    // Takes in user progress on anime show and updates table
    if user_progress == "cmp" {
        let _my_status = AnimeStatus::Completed;
    } else if user_progress == "ip" {
        let _my_status = AnimeStatus::InProgress;
    } else if user_progress == "drp" {
        let _my_status = AnimeStatus::Dropped;
    } else {
        let _my_status = AnimeStatus::PlanOnWatching;
    }

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
    let mut anime_table = Table::new();
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

    println!("Do want to enter a title: ");
    let mut add_more = String::new();
    let _ = io::stdin().read_line(&mut add_more);
    let add_more = add_more.trim();

    if add_more == "y" {
        loop {
            //Write and save to file
            let file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("anime.csv")?; // this only work if you are in src directory
            //TODO! //check current directory and move into src if not
            //already in src

            let mut wtr = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);

            let sheet = fill_table();
            wtr.serialize(&sheet)?;
            wtr.flush()?;

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
                break;
            }
        }
    } else {
        println!("{anime_table}");
    } // end of loop
    println!("{anime_table}");
    Ok(())
}

// Update the table manually by directly opening the csv file
pub fn update_table() {
    println!("Would you like to update any current entries in the table: ");
    let mut update_entries = String::new();
    let _ = io::stdin().read_line(&mut update_entries);
    let update_entries = update_entries.trim();
    if update_entries == "y" {
        let _ = Command::new("xdg-open").arg("anime.csv").spawn();
    } else {
        exit(0);
    }
}

// View table
pub fn print_table() -> Result<(), Box<dyn std::error::Error>> {
    // Open csv file and read each row
    let open_file = File::open("anime.csv")?;
    let mut rdr = Reader::from_reader(open_file);
    let mut anime_data = Vec::new();

    // Go through each row and the data into the vec
    for final_table in rdr.records() {
        let record = final_table?;
        anime_data.push(record);
    }

    // Create a table
    let mut anime_table = Table::new();
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

    // Iterates through row in the csv
    // Add data from csv to each row in table
    for record in &anime_data {
        anime_table.add_row(record);
    }
    // View and print the table
    println!("{anime_table}");
    Ok(())
}

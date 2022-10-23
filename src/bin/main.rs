use std::io;
use chrono::Datelike;

/*
    MLB Stat Searcher

    Basically, I &&want this CLI app to be used in the following way:

    1. cargo run
    2. Prompt user for a search query (i.e. player's name)
    3. Make the API call to the MLB data API to search for that (goes in the query string "name_part")
    4. Query should output either multiple, one, or no players
        4.1 If no players, print appropriate message and return
        4.2 If multiple players, list out all players with (name, team, position) and prompt user to pick one. Then move on as if that is the one player

    5. Grab the player ID and make another API call to get the player's stats for the year
    6. Display stats in a pretty manner (i.e. (player name, team, position, AVG/OBP/SLG, etc))
    7. Exit
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_year = chrono::Utc::now().year();

    println!("Enter season year (e.g. 2022): ");
    let mut season = String::new();

    loop {
        read_input(&mut season);

        // Limit the season to 1871, though from manual testing, MLB only has earliest stats to 1876
        match season.trim() {
            "" => {
                println!("Using default year (this year)");
                season.push_str(&current_year.to_string());
                break;
            }
            season => match season.parse::<i32>() {
                Ok(num) => {
                    if num < 1871 || num > current_year {
                        println!("Year must be between years 1871 and {}", current_year);
                    } else {
                        break;
                    }
                }
                Err(_) => println!("Not a valid year"),
            }
        }

        season.clear();
    }

    let season = season.trim();

    let mlb_client = mlb::MlbClient::new(season);

    // TODO: better option handling, this code is for player search
    // println!("Enter name of player to search: ");
    // let mut name_query = String::new();
    // read_input(&mut name_query);

    // let player = mlb_client.get_player(&name_query)?;
    // println!("Printing statline for player...");
    // player.print_statline();

    // TODO: better option handling, this code is for leaders + team stats
    // mlb_client.get_team_stats();
    // mlb_client.get_stat_leaders("hitting");
    // mlb_client.get_team_stat_leaders("pitching");

    Ok(())
}

fn read_input(line: &mut String) {
    io::stdin().read_line(line).expect("Failed to read line");
}

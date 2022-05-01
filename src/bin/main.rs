use std::io;

/*
    MLB Stat Searcher

    Basically, I want this CLI app to be used in the following way:

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
    let mlb_client = mlb::MlbClient::new();

    println!("Enter name of player to search: ");
    let mut name_query = String::new();
    io::stdin()
        .read_line(&mut name_query)
        .expect("Failed to read line");

    mlb_client.get_player(&name_query)?;

    Ok(())
}

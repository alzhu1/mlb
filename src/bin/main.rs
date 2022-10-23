use std::io;

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
    let choice = get_entry();
    let mlb_client = mlb::MlbClient::new();

    match choice.as_str() {
        "1" => mlb_client.get_player(),
        "2" => mlb_client.get_team_stats(),
        "3" => mlb_client.get_stat_leaders("hitting"),
        "4" => mlb_client.get_team_stat_leaders("pitching"),
        _ => panic!("This should not execute"),
    }

    Ok(())
}

fn read_input(line: &mut String) {
    io::stdin().read_line(line).expect("Failed to read line");
}

fn get_entry() -> String{
    println!("Entering MLB Client. Select action:");

    // Listing options
    let mut choice = String::new();
    loop {
        println!("1) Search for a player");
        println!("2) Get team stats");
        println!("3) Get stat leaders");
        println!("4) Get team stat leaders");

        read_input(&mut choice);
        match choice.trim() {
            "1" | "2" | "3" | "4" => break,
            _ => println!("Select an option between 1-4"),
        }

        choice.clear();
    }

    choice.trim().to_string()
}
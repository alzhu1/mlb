use std::io;

/*
    MLB Stat Searcher

    CLI app that can look up stats for players/teams in a season
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let choice = get_entry();
    let mut mlb_client = mlb::create_client(io::stdin().lock());

    match choice.as_str() {
        "1" => mlb_client.get_player(),
        "2" => mlb_client.get_team_stats(),
        "3" => mlb_client.get_stat_leaders(),
        "4" => mlb_client.get_team_stat_leaders(),
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
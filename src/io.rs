use chrono::Datelike;
use std::{collections::HashMap, io::BufRead};
use serde_json::Value;

pub struct IOReader<R> {
    pub reader: R,
}

impl<R> IOReader<R>
where
    R: BufRead
{
    fn read_input(&mut self, line: &mut String) {
        self.reader.read_line(line).expect("Failed to read line");
    }

    pub fn get_season(&mut self) -> String {
        let current_year = chrono::Utc::now().year();
        println!("Enter season year (e.g. 2022): ");
        let mut season = String::new();

        loop {
            self.read_input(&mut season);

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
        season.trim().to_string()
    }

    pub fn get_name_query(&mut self) -> String {
        println!("Enter name of player to search: ");
        let mut name_query = String::new();
        self.read_input(&mut name_query);

        name_query.trim().to_owned()
    }

    pub fn get_filtered_players<'a>(&'a mut self, team_id_map: &'a HashMap<u64, String>, filtered_players: &'a Vec<&'a Value>) -> Option<&'a Value> {
        println!(
            "{} players found, select the player to view stats for (pick a number).",
            filtered_players.len()
        );

        for (index, player) in filtered_players.iter().enumerate() {
            println!(
                "{}) {}, {} ({})",
                index + 1,
                player["fullName"].as_str().unwrap(),
                team_id_map[&player["currentTeam"]["id"].as_u64().unwrap()],
                player["primaryPosition"]["abbreviation"].as_str().unwrap()
            );
        }

        let mut chosen_player = String::new();
        self.read_input(&mut chosen_player);

        Some(filtered_players[chosen_player.trim().parse::<usize>().unwrap() - 1])
    }

    pub fn get_team_id(&mut self, team_id_map: &HashMap<u64, String>) -> u64 {
        println!("Select a team:");

        let mut team_ids: Vec<&u64> = Vec::new();
        for (index, (key, value)) in team_id_map.iter().enumerate() {
            print!("{}) {: <10}", index + 1, value);
            print!("{}", if index  % 5 == 4 { "\n" } else { "\t" });

            team_ids.push(key);
        }

        let mut chosen_team = String::new();
        self.read_input(&mut chosen_team);

        team_ids[chosen_team.trim().parse::<usize>().unwrap() - 1].to_owned()
    }

    pub fn get_leader_category<'a>(&'a mut self, leader_categories: &'a HashMap<&'a str, &'a str>, stat_categories: &'a [&'a str]) -> &'a str {
        println!("Select a leader category:");
        for (index, category) in stat_categories.iter().enumerate() {
            print!("{}) {: <10}", index + 1, category);
            print!("{}", if index % 3 == 2 || index == stat_categories.len() { "\n" } else { "\t" });
        }

        let mut chosen_category = String::new();
        self.read_input(&mut chosen_category);
        let chosen_category = stat_categories[chosen_category.trim().parse::<usize>().unwrap() - 1].to_owned();
        leader_categories[chosen_category.as_str()]
    }

    pub fn get_stat_type(&mut self) -> String {
        println!("Enter stat type for leaders (hitting or pitching): ");
        let mut stat_type = String::new();
        self.read_input(&mut stat_type);

        stat_type.trim().to_owned()
    }
}

// TODO: look into parameterized testing options, rust macros don't feel good enough to use
#[cfg(test)]
mod test {
    use super::IOReader;

    #[test]
    fn test_get_season_success() {
        let input = "2022\n".as_bytes();
        let mut io_reader = IOReader { reader: input };

        assert_eq!("2022", io_reader.get_season());
    }

    #[test]
    fn test_get_season_retry() {
        let input = "1850\n2099\n2010\n".as_bytes();
        let mut io_reader = IOReader { reader: input };

        assert_eq!("2010", io_reader.get_season());
    }

    #[test]
    fn test_get_name_query() {
        let input = "Alvarez\n".as_bytes();
        let mut io_reader = IOReader { reader: input };

        assert_eq!("Alvarez", io_reader.get_name_query());
    }

    // NOTE: Could add more testing for functions here. Given the nature of these funcs though, it's a lot of setup
    // Will leave this for now
}
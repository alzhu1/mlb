use std::{collections::HashMap, io};

use reqwest::Url;
use serde_json::Value;

const MLB_LOOKUP_API_ENDPOINT: &str = "https://statsapi.mlb.com/api/v1";

const TEAMS_LOOKUP: &str = "teams";
const SEARCH_PLAYER_ALL: &str = "sports/1/players";

fn get_formatted_url(path: &str) -> Result<Url, Box<dyn std::error::Error>> {
    Ok(Url::parse(format!("{}/{}", MLB_LOOKUP_API_ENDPOINT, path).as_str())?)
}

fn get(path: &str, query_params: HashMap<&str, &str>) -> Result<Value, Box<dyn std::error::Error>> {
    let mut url = get_formatted_url(path)?;

    // Add query params, drop mutable ref to iterator once done
    {
        let mut url_query_params = url.query_pairs_mut();

        for (key, value) in query_params {
            url_query_params.append_pair(key, value);
        }
    }

    Ok(reqwest::blocking::get(url.as_str())?.json::<Value>()?)
}

pub struct MlbClient {
    team_id_map: HashMap<u64, String>,
}

impl MlbClient {
    pub fn new() -> Self {
        let team_resp = get(TEAMS_LOOKUP, HashMap::from([
            ("sportId", "1"),
            ("fields", "teams,id,abbreviation"),
        ]));

        let team_resp = match team_resp {
            Ok(resp) => resp,
            Err(e) => panic!("Failed to get response: {}", e),
        };

        let team_id_map: HashMap<u64, String> = team_resp["teams"]
            .as_array()
            .unwrap()
            .iter()
            .map(|team| {
                (
                    team["id"].as_u64().unwrap(),
                    String::from(team["abbreviation"].as_str().unwrap()),
                )
            })
            .collect();

        println!("{:#?}", team_id_map.len());

        MlbClient { team_id_map }
    }

    pub fn get_player(&self, name_query: &str) -> Result<(), Box<dyn std::error::Error>> {
        let resp = get(SEARCH_PLAYER_ALL, HashMap::from([("season", "2022")]))?;
        let players = resp["people"].as_array().unwrap();

        let filtered_players: Vec<&Value> = players
            .iter()
            .filter(|player| {
                player["fullName"]
                    .as_str()
                    .unwrap()
                    .to_lowercase()
                    .contains(name_query.trim())
            })
            .collect();

        let player: Option<&Value> = match filtered_players.len() {
            0 => None,
            1 => Some(filtered_players[0]),
            _ => {
                println!(
                    "{} players found, select the player to view stats for (pick a number).",
                    filtered_players.len()
                );

                for (index, player) in filtered_players.iter().enumerate() {
                    println!(
                        "{}) {}, {} ({})",
                        index + 1,
                        player["fullName"].as_str().unwrap(),
                        self.team_id_map[&player["currentTeam"]["id"].as_u64().unwrap()],
                        player["primaryPosition"]["abbreviation"].as_str().unwrap()
                    );
                }

                let mut chosen_player = String::new();
                io::stdin()
                    .read_line(&mut chosen_player)
                    .expect("Failed to read line");

                Some(filtered_players[chosen_player.trim().parse::<usize>().unwrap() - 1])
            }
        };

        println!("{:#?}", player);

        Ok(())
    }
}

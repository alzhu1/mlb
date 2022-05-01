use std::{collections::HashMap, io};

use reqwest::Url;
use serde_json::Value;

const MLB_LOOKUP_API_ENDPOINT: &str = "https://statsapi.mlb.com/api/v1";

const TEAMS_LOOKUP: &str = "teams";
const SEARCH_PLAYER_ALL: &str = "sports/1/players";

pub struct MlbClient {
    pub team_id_map: HashMap<u64, String>,
}

impl MlbClient {
    pub fn new() -> Self {
        let team_lookup_url =
            Url::parse(format!("{}/{}", MLB_LOOKUP_API_ENDPOINT, TEAMS_LOOKUP).as_str());

        let mut team_lookup_url = match team_lookup_url {
            Ok(url) => url,
            Err(e) => panic!("Failed to parse url: {}", e),
        };

        team_lookup_url
            .query_pairs_mut()
            .append_pair("sportId", "1")
            .append_pair("fields", "teams,id,abbreviation");

        let team_resp = reqwest::blocking::get(team_lookup_url.as_str());
        let team_resp = match team_resp {
            Ok(resp) => resp,
            Err(e) => panic!("Failed to get response: {}", e),
        };

        let team_resp = team_resp.json::<Value>().unwrap();

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

        MlbClient { team_id_map }
    }

    pub fn get_player(&self, name_query: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut url =
            Url::parse(format!("{}/{}", MLB_LOOKUP_API_ENDPOINT, SEARCH_PLAYER_ALL).as_str())?;

        url.query_pairs_mut().append_pair("season", "2022");

        let resp = reqwest::blocking::get(url.as_str())?.json::<serde_json::Value>()?;
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

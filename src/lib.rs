mod requests;

use std::{collections::HashMap, io};

use requests::{get_player_details, get_teams, get_players};
use serde::Deserialize;
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};

pub struct MlbClient {
    season: String,
    team_id_map: HashMap<u64, String>,
}

impl MlbClient {
    pub fn new(season: &str) -> Self {
        let team_resp = get_teams(season);

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

        MlbClient {
            season: season.to_string(),
            team_id_map,
        }
    }

    pub fn get_player(
        &self,
        name_query: &str,
    ) -> Result<Box<dyn Player>, Box<dyn std::error::Error>> {
        let resp = get_players(&self.season.as_str());
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

        let player_value: Option<&Value> = match filtered_players.len() {
            0 => None,
            1 => Some(filtered_players[0]),
            _ => self.filter_players(&filtered_players)
        };

        let player: Option<Box<dyn Player>> = if let Some(player_value) = player_value {
            let player_id = player_value["id"].as_u64().unwrap();

            match player_value["primaryPosition"]["abbreviation"]
                .as_str()
                .unwrap()
            {
                "P" => Some(Box::new(Pitcher::new(player_id, &self.season))),
                "TWP" => Some(Box::new(Pitcher::new(player_id, &self.season))), // TODO: fix for shohei
                _ => Some(Box::new(Batter::new(player_id, &self.season))),
            }
        } else {
            None
        };

        Ok(player.unwrap())
    }

    fn filter_players<'a>(&'a self, filtered_players: &'a Vec<&Value>) -> Option<&Value> {
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
}

pub trait Player: std::fmt::Debug {
    fn print_statline(&self);
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Batter {
    gamesPlayed: u64,
    plateAppearances: u64,
    atBats: u64,
    runs: u64,
    hits: u64,
    doubles: u64,
    triples: u64,
    homeRuns: u64,
    rbi: u64,
    stolenBases: u64,
    caughtStealing: u64,
    baseOnBalls: u64,
    hitByPitch: u64,
    strikeOuts: u64,
    #[serde_as(as = "DisplayFromStr")]
    avg: f64,
    #[serde_as(as = "DisplayFromStr")]
    babip: f64,
    #[serde_as(as = "DisplayFromStr")]
    obp: f64,
    #[serde_as(as = "DisplayFromStr")]
    slg: f64,
    #[serde_as(as = "DisplayFromStr")]
    ops: f64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Pitcher {
    wins: u64,
    losses: u64,

    // TODO: this is expressed as ".---" for players with 0 W/ 0L
    // #[serde_as(as = "DisplayFromStr")]
    // winPercentage: f64,
    #[serde_as(as = "DisplayFromStr")]
    era: f64,
    gamesPitched: u64,
    gamesStarted: u64,
    gamesFinished: u64,
    completeGames: u64,
    shutouts: u64,
    holds: u64,
    saves: u64,
    #[serde_as(as = "DisplayFromStr")]
    inningsPitched: f64,
    hits: u64,
    runs: u64,
    earnedRuns: u64,
    homeRuns: u64,
    baseOnBalls: u64,
    strikeOuts: u64,
    hitByPitch: u64,
    #[serde_as(as = "DisplayFromStr")]
    whip: f64,
    #[serde_as(as = "DisplayFromStr")]
    hitsPer9Inn: f64,
    #[serde_as(as = "DisplayFromStr")]
    homeRunsPer9: f64,
    #[serde_as(as = "DisplayFromStr")]
    walksPer9Inn: f64,
    #[serde_as(as = "DisplayFromStr")]
    strikeoutsPer9Inn: f64,
    #[serde_as(as = "DisplayFromStr")]
    strikeoutWalkRatio: f64,
}

impl Batter {
    fn new(player_id: u64, season: &str) -> Self {
        let mut player = get_player_details(player_id, "hitting", season);
        let stats = player["people"][0]["stats"][0]["splits"][0]["stat"].take();
        serde_json::from_value(stats).unwrap()
    }
}

impl Pitcher {
    fn new(player_id: u64, season: &str) -> Self {
        let mut player = get_player_details(player_id, "pitching", season);
        let stats = player["people"][0]["stats"][0]["splits"][0]["stat"].take();
        serde_json::from_value(stats).unwrap()
    }
}

impl Player for Batter {
    fn print_statline(&self) {
        println!("{:.3}/{:.3}/{:.3}", self.avg, self.obp, self.slg);
    }
}

impl Player for Pitcher {
    fn print_statline(&self) {
        println!("{} W-{} L, {} ERA", self.wins, self.losses, self.era);
    }
}

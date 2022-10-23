mod requests;

use std::{collections::HashMap, io};

use requests::{get_player_details, get_teams, get_players, get_team_stats};
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
        let resp = get_players(&self.season);
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

    pub fn get_team_stats(&self) {
        println!("Select a team:");

        let mut index = 0;
        let selection_to_team_id_map: HashMap<u64, &u64> =self.team_id_map.iter()
            .map(|(key, value)| {
                index += 1;
                print!("{}) {}", index, value);
                print!("{}", if index % 5 == 0 { "\n" } else { "\t\t" });

                (index, key)
            })
            .collect();

        let mut chosen_team = String::new();
            io::stdin()
                .read_line(&mut chosen_team)
                .expect("Failed to read line");

        let chosen_team = selection_to_team_id_map[&chosen_team.trim().parse::<u64>().unwrap()].to_owned();
        let resp = get_team_stats(chosen_team, &self.season);
        let stats = resp["stats"].as_array().unwrap();

        let hitting_stats: &Value;
        let pitching_stats: &Value;
        if stats[0]["group"]["displayName"].as_str().unwrap().eq("hitting") {
            hitting_stats = &stats[0]["splits"].as_array().unwrap()[0];
            pitching_stats = &stats[1]["splits"].as_array().unwrap()[0];
        } else {
            pitching_stats = &stats[0]["splits"].as_array().unwrap()[0];
            hitting_stats = &stats[1]["splits"].as_array().unwrap()[0];
        }
        let team_name: &str = hitting_stats["team"]["name"].as_str().unwrap();

        // Print hitting stats
        println!("\n{} Hitting:", team_name);
        println!(
            "{: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10}",
            "R", "H", "2B", "3B", "HR", "SB", "CS", "BB", "HBP", "IBB", "SO", "BA", "OBP", "SLG", "OPS"
        );
        println!(
            "{: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10}",
            hitting_stats["stat"]["runs"].as_u64().unwrap(),
            hitting_stats["stat"]["hits"].as_u64().unwrap(),
            hitting_stats["stat"]["doubles"].as_u64().unwrap(),
            hitting_stats["stat"]["triples"].as_u64().unwrap(),
            hitting_stats["stat"]["homeRuns"].as_u64().unwrap(),
            hitting_stats["stat"]["stolenBases"].as_u64().unwrap(),
            hitting_stats["stat"]["caughtStealing"].as_u64().unwrap(),
            hitting_stats["stat"]["baseOnBalls"].as_u64().unwrap(),
            hitting_stats["stat"]["hitByPitch"].as_u64().unwrap(),
            hitting_stats["stat"]["intentionalWalks"].as_u64().unwrap(),
            hitting_stats["stat"]["strikeOuts"].as_u64().unwrap(),
            hitting_stats["stat"]["avg"].as_str().unwrap(),
            hitting_stats["stat"]["obp"].as_str().unwrap(),
            hitting_stats["stat"]["slg"].as_str().unwrap(),
            hitting_stats["stat"]["ops"].as_str().unwrap(),
        );

        // Print pitching stats
        println!("\n{} Pitching:", team_name);
        println!(
            "{: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10}",
            "W", "L", "W-L%", "ERA", "CG", "SHO", "HLD", "SV", "IP", "HR", "BB", "SO", "HBP", "WHIP", "HR9", "BB9", "SO9", "SO/W"
        );

        println!(
            "{: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10} | {: <10}",
            pitching_stats["stat"]["wins"].as_u64().unwrap(),
            pitching_stats["stat"]["losses"].as_u64().unwrap(),
            pitching_stats["stat"]["winPercentage"].as_str().unwrap(),
            pitching_stats["stat"]["era"].as_str().unwrap(),
            pitching_stats["stat"]["completeGames"].as_u64().unwrap(),
            pitching_stats["stat"]["shutouts"].as_u64().unwrap(),
            pitching_stats["stat"]["holds"].as_u64().unwrap(),
            pitching_stats["stat"]["saves"].as_u64().unwrap(),
            pitching_stats["stat"]["inningsPitched"].as_str().unwrap(),
            pitching_stats["stat"]["homeRuns"].as_u64().unwrap(),
            pitching_stats["stat"]["baseOnBalls"].as_u64().unwrap(),
            pitching_stats["stat"]["strikeOuts"].as_u64().unwrap(),
            pitching_stats["stat"]["hitByPitch"].as_u64().unwrap(),
            pitching_stats["stat"]["whip"].as_str().unwrap(),
            pitching_stats["stat"]["homeRunsPer9"].as_str().unwrap(),
            pitching_stats["stat"]["walksPer9Inn"].as_str().unwrap(),
            pitching_stats["stat"]["strikeoutsPer9Inn"].as_str().unwrap(),
            pitching_stats["stat"]["strikeoutWalkRatio"].as_str().unwrap(),
        );
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

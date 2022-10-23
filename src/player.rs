use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::requests::get_player_details;

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
    pub fn new(player_id: u64, season: &str) -> Self {
        let mut player = get_player_details(player_id, "hitting", season);
        let stats = player["people"][0]["stats"][0]["splits"][0]["stat"].take();
        serde_json::from_value(stats).unwrap()
    }
}

impl Pitcher {
    pub fn new(player_id: u64, season: &str) -> Self {
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
use std::collections::HashMap;
use reqwest::Url;
use serde_json::Value;

const MLB_LOOKUP_API_ENDPOINT: &str = "https://statsapi.mlb.com/api/v1";

const PLAYER_LOOKUP: &str = "people";
const TEAMS_LOOKUP: &str = "teams";
const SEARCH_PLAYER_ALL: &str = "sports/1/players";
const STAT_LEADERS: &str = "stats/leaders";

fn get_formatted_url(path: &str) -> Result<Url, Box<dyn std::error::Error>> {
    Ok(Url::parse(
        format!("{}/{}", MLB_LOOKUP_API_ENDPOINT, path).as_str(),
    )?)
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

pub fn get_teams(season: &str) -> Value {
    let resp = get(
        TEAMS_LOOKUP,
        HashMap::from([
            ("sportId", "1"),
            ("fields", "teams,id,abbreviation"),
            ("season", season),
        ]),
    );

    match resp {
        Ok(response) => response,
        Err(e) => panic!("Failed to get response: {}", e),
    }
}

pub fn get_players(season: &str) -> Value {
    let resp = get(
        SEARCH_PLAYER_ALL,
        HashMap::from([("season", season)]),
    );

    match resp {
        Ok(response) => response,
        Err(e) => panic!("Failed to get response: {}", e),
    }
}

pub fn get_player_details(player_id: u64, player_type: &str, season: &str) -> Value {
    let resp = get(
        PLAYER_LOOKUP,
        HashMap::from([
            ("personIds", player_id.to_string().as_str()),
            (
                "hydrate",
                format!(
                    "stats(group=[{}],type=season,season={}),currentTeam",
                    player_type, season
                )
                .as_str(),
            ),
        ]),
    );

    match resp {
        Ok(response) => response,
        Err(e) => panic!("Failed to get response: {}", e),
    }
}

pub fn get_team_stats(team_id: u64, season: &str) -> Value {
    let path = format!("{}/{}/stats", TEAMS_LOOKUP, team_id);
    let resp = get(
        path.as_str(),
        HashMap::from([
            ("season", season),
            ("group", "hitting,pitching"),
            ("stats", "season"),
        ]),
    );

    match resp {
        Ok(response) => response,
        Err(e) => panic!("Failed to get response: {}", e),
    }
}

pub fn get_stat_leaders(leader_categories: &str, stat_type: &str, season: &str) -> Value {
    let resp = get(
        STAT_LEADERS,
        HashMap::from([
            ("season", season),
            ("statGroup", stat_type),
            ("leaderCategories", leader_categories),
        ]),
    );

    match resp {
        Ok(response) => response,
        Err(e) => panic!("Failed to get response: {}", e),
    }
}

pub fn get_team_stat_leaders(team_id: u64, leader_categories: &str, season: &str) -> Value {
    let path = format!("{}/{}/leaders", TEAMS_LOOKUP, team_id);
    let resp = get(
        path.as_str(),
        HashMap::from([
            ("season", season),
            ("leaderCategories", leader_categories),
        ]),
    );

    match resp {
        Ok(response) => response,
        Err(e) => panic!("Failed to get response: {}", e),
    }
}
use mlb;

// TODO: Capture output, it's complicated to do now

#[test]
fn test_get_batter() {
    let input = ["2022", "Joey Bart"].join("\n") + "\n";
    mlb::create_client(input.as_bytes()).get_player();
}

#[test]
fn test_get_pitcher() {
    let input = ["2022", "Logan Webb"].join("\n") + "\n";
    mlb::create_client(input.as_bytes()).get_player();
}

#[test]
fn test_get_multiple_players() {
    let input = ["2022", "Will Smith", "1"].join("\n") + "\n";
    mlb::create_client(input.as_bytes()).get_player();
}

#[test]
fn test_get_team_stats() {
    let input = ["2022", "1"].join("\n") + "\n";
    mlb::create_client(input.as_bytes()).get_team_stats();
}

#[test]
fn test_get_hitting_stat_leaders() {
    let input = ["2022", "hitting", "2"].join("\n") + "\n"; // HRs
    mlb::create_client(input.as_bytes()).get_stat_leaders();
}

#[test]
fn test_get_pitching_stat_leaders() {
    let input = ["2022", "pitching", "3"].join("\n") + "\n"; // ERA
    mlb::create_client(input.as_bytes()).get_stat_leaders();
}

#[test]
fn test_get_hitting_team_stat_leaders() {
    let input = ["2022", "hitting", "1", "2"].join("\n") + "\n"; // HRs
    mlb::create_client(input.as_bytes()).get_team_stat_leaders();
}

#[test]
fn test_get_pitching_team_stat_leaders() {
    let input = ["2022", "pitching", "1", "3"].join("\n") + "\n"; // ERA
    mlb::create_client(input.as_bytes()).get_team_stat_leaders();
}
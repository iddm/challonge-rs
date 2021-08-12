//! Challonge Tournament type.

extern crate serde_json;

use chrono::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use error::Error;
use util::{decode_array, into_map, remove};

/// Tournament includes.
#[derive(Debug, Clone)]
pub enum TournamentIncludes {
    /// Includes matches and participants
    All,

    /// Includes matches
    Matches,

    /// Includes participants
    Participants,
}

/// Tournament ranking order.
#[derive(Debug, Clone)]
pub enum RankedBy {
    /// Rank by number of matches won
    MatchWins,

    /// Rank by number of games won
    GameWins,

    /// Rank by points scored
    PointsScored,

    /// Rank by difference in points
    PointsDifference,

    /// Custom ranking rules
    Custom,
}
impl fmt::Display for RankedBy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RankedBy::MatchWins => {
                fmt.write_str("match wins")?;
            }
            RankedBy::GameWins => {
                fmt.write_str("game wins")?;
            }
            RankedBy::PointsScored => {
                fmt.write_str("points scored")?;
            }
            RankedBy::PointsDifference => {
                fmt.write_str("points difference")?;
            }
            RankedBy::Custom => {
                fmt.write_str("custom")?;
            }
        }
        Ok(())
    }
}

/// Tournament ID is an integer value or pair of strings (subdomain and tournament url)
#[derive(Debug, Clone, PartialEq)]
pub enum TournamentId {
    /// Subdomain and Tournament url
    Url(String, String),

    /// Unique idenfifier (number) in challonge system
    Id(u64),
}
impl fmt::Display for TournamentId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TournamentId::Url(ref subdomain, ref tournament_url) => {
                if subdomain.is_empty() {
                    fmt.write_str(tournament_url)?;
                } else {
                    fmt.write_str(&format!("{}-{}", subdomain, tournament_url))?;
                }
            }
            TournamentId::Id(ref id) => {
                fmt.write_str(&id.to_string())?;
            }
        }
        Ok(())
    }
}

/// Game points definition.
#[derive(Debug, Clone, PartialEq)]
pub struct GamePoints {
    /// Points for winning a match
    pub match_win: f64,

    /// Points for tie match
    pub match_tie: f64,

    /// Points for winning a game
    pub game_win: f64,

    /// Points for a tie game
    pub game_tie: f64,

    /// ??? Points for exiting the tournament ???
    pub bye: Option<f64>,
}
impl GamePoints {
    /// Creates new `GamePoints` with default values.
    pub fn new(
        match_win: f64,
        match_tie: f64,
        game_win: f64,
        game_tie: f64,
        bye: Option<f64>,
    ) -> GamePoints {
        GamePoints {
            match_win,
            match_tie,
            game_win,
            game_tie,
            bye,
        }
    }

    /// Decode `GamePoints` from JSON.
    pub fn decode(
        mut map: &mut BTreeMap<String, Value>,
        prefix: &str,
    ) -> Result<GamePoints, Error> {
        let mut bye = None;
        if let Ok(bye_pts) = remove(&mut map, &format!("{}pts_for_bye", prefix)) {
            if let Ok(b) = bye_pts.as_string().unwrap_or("").to_owned().parse::<f64>() {
                bye = Some(b);
            }
        }

        Ok(GamePoints {
            match_win: remove(&mut map, &format!("{}pts_for_match_win", prefix))?
                .as_string()
                .unwrap_or("")
                .to_owned()
                .parse::<f64>()
                .unwrap_or(0f64),
            match_tie: remove(&mut map, &format!("{}pts_for_match_tie", prefix))?
                .as_string()
                .unwrap_or("")
                .to_owned()
                .parse::<f64>()
                .unwrap_or(0f64),
            game_win: remove(&mut map, &format!("{}pts_for_game_win", prefix))?
                .as_string()
                .unwrap_or("")
                .to_owned()
                .parse::<f64>()
                .unwrap_or(0f64),
            game_tie: remove(&mut map, &format!("{}pts_for_game_tie", prefix))?
                .as_string()
                .unwrap_or("")
                .to_owned()
                .parse::<f64>()
                .unwrap_or(0f64),
            bye,
        })
    }
}
impl Default for GamePoints {
    fn default() -> GamePoints {
        GamePoints {
            match_win: 0.5f64,
            match_tie: 1.0f64,
            game_win: 0.0f64,
            game_tie: 0.0f64,
            bye: None,
        }
    }
}

/// Structure for creating a tournament.
#[derive(Debug, Clone)]
pub struct TournamentCreate {
    /// Your event's name/title (Max: 60 characters)
    pub name: String,

    /// Type of a tournament
    pub tournament_type: TournamentType,

    /// challonge.com/url (letters, numbers, and underscores only)
    pub url: String,

    /// subdomain.challonge.com/url (Requires write access to the specified subdomain)
    pub subdomain: String,

    /// Description/instructions to be displayed above the bracket
    pub description: String,

    /// Have Challonge host a sign-up page (otherwise, you manually add all participants)
    pub open_signup: bool,

    /// Single Elimination only
    pub hold_third_place_match: bool,

    /// Only for Swiss system
    pub swiss_points: GamePoints,

    /// Number of rounds in swiss system
    pub swiss_rounds: u64,

    /// Tournament ranking type
    pub ranked_by: RankedBy,

    /// Only for Round Robin system
    pub round_robin_points: GamePoints,

    /// Single &amp; Double Elimination only - Label each round above the bracket (default: false)
    pub show_rounds: bool,

    /// Hide this tournament from the public browsable index and your profile (default: false)
    pub private: bool,

    /// Name of the game to which this tournament belongs to.
    pub game_name: Option<String>,

    /// Email registered Challonge participants when matches open up for them (default: false)
    pub notify_users_when_matches_open: bool,

    /// Email registered Challonge participants the results when this tournament ends (default: false)
    pub notify_users_when_the_tournament_ends: bool,

    /// Instead of traditional seeding rules, make pairings by going straight down the list of participants.
    /// First round matches are filled in top to bottom, then qualifying matches (if applicable). (default: false)
    pub sequential_pairings: bool,

    /// Maximum number of participants in the bracket.
    /// A waiting list (attribute on Participant) will capture participants once the cap is reached.
    pub signup_cap: u64,

    /// the planned or anticipated start time for the tournament (Used with check_in_duration to determine participant check-in window). Timezone defaults to Eastern.
    pub start_at: Option<DateTime<Utc>>,

    /// Length of the participant check-in window in minutes.
    pub check_in_duration: u64,

    /// This option only affects double elimination. null/blank (default) - give the winners bracket finalist two chances to beat the losers bracket finalist, 'single match' - create only one grand finals match, 'skip' - don't create a finals match between winners and losers bracket finalists
    pub grand_finals_modifier: Option<String>,
}
impl TournamentCreate {
    /// Creates new `TournamentCreate` structure with default values.
    pub fn new() -> TournamentCreate {
        TournamentCreate {
            name: String::default(),
            tournament_type: TournamentType::SingleElimination,
            url: String::default(),
            subdomain: String::default(),
            description: String::default(),
            open_signup: false,
            hold_third_place_match: false,
            swiss_points: GamePoints::new(0.5f64, 1.0f64, 0.0f64, 0.0f64, Some(0.0f64)),
            swiss_rounds: 0,
            ranked_by: RankedBy::PointsScored,
            round_robin_points: GamePoints::default(),
            show_rounds: false,
            private: false,
            game_name: None,
            notify_users_when_matches_open: true,
            notify_users_when_the_tournament_ends: true,
            sequential_pairings: false,
            signup_cap: 4,
            start_at: None,
            check_in_duration: 60,
            grand_finals_modifier: None,
        }
    }

    builder_s!(name);
    builder!(tournament_type, TournamentType);
    builder_s!(url);
    builder_s!(subdomain);
    builder_s!(description);
    builder!(open_signup, bool);
    builder!(hold_third_place_match, bool);
    builder!(swiss_points, GamePoints);
    builder!(swiss_rounds, u64);
    builder!(ranked_by, RankedBy);
    builder!(round_robin_points, GamePoints);
    builder!(show_rounds, bool);
    builder!(private, bool);
    builder_so!(game_name);
    builder!(notify_users_when_matches_open, bool);
    builder!(notify_users_when_the_tournament_ends, bool);
    builder!(sequential_pairings, bool);
    builder!(signup_cap, u64);
    builder!(check_in_duration, u64);
    builder!(grand_finals_modifier, Option<String>);
}

impl Default for TournamentCreate {
    fn default() -> Self {
        Self::new()
    }
}

/// Challonge `Tournament` definition.
#[derive(Debug, Clone)]
pub struct Tournament {
    /// Tournament may have attachments
    pub accept_attachments: bool,

    /// Participants are able to report stats of the match by themselves
    pub allow_participant_match_reporting: bool,

    /// Tournament supports anonymous voting
    pub anonymous_voting: bool,
    // category: ??,
    // check_in_duration: ??,
    // completed_at: ??,
    /// Time when the tournament was created
    pub created_at: DateTime<FixedOffset>,

    /// `true` if created by the API
    pub created_by_api: bool,

    /// ???
    pub credit_capped: bool,

    /// Description of the tournament
    pub description: String,

    /// An id of the game the tournament belongs to
    pub game_id: u64,

    /// Tournament has group stages enabled
    pub group_stages_enabled: bool,

    /// Hide forums from users
    pub hide_forum: bool,

    /// Hide seeds from users
    pub hide_seeds: bool,

    /// ???
    pub hold_third_place_match: bool,

    /// Unique tournament identifier in challonge system
    pub id: TournamentId,

    /// Maximum number of predictions for each user
    pub max_predictions_per_user: u64,

    /// Name of the tournament
    pub name: String,

    /// Should challonge system notify registered users when the matches available
    pub notify_users_when_matches_open: bool,
    /// Should challonge system notify registered users when the tournament has come to end
    pub notify_users_when_the_tournament_ends: bool,

    /// Are signups open
    pub open_signup: bool,

    /// Number of participants of the tournament
    pub participants_count: u64,

    /// ???
    pub prediction_method: u64,
    // <predictions-opened-at nil="true"/>
    /// ???
    pub private: bool,

    /// ???
    pub progress_meter: u64,

    /// A points for matches/games in swiss system
    pub swiss_points: GamePoints,

    /// ???
    pub quick_advance: bool,
    // <ranked-by>match wins</ranked-by>
    /// Tournament will require score agreement from all of participants of the match
    pub require_score_agreement: bool,

    /// A points for matches/games in round robin system
    pub round_robin_points: GamePoints,

    /// ???
    pub sequential_pairings: bool,

    /// Show rounds on the web page
    pub show_rounds: bool,
    // <signup-cap nil="true"/>
    // <start-at nil="true"/>
    //
    /// Time when the tournament was started
    pub started_at: Option<DateTime<FixedOffset>>, //2015-01-19T16:57:17-05:00</started-at>
    // <started-checking-in-at nil="true"/>
    // <state>underway</state>
    /// Number of rounds in swiss system
    pub swiss_rounds: u64,

    /// The tournament works with teams
    pub teams: bool,
    // <tie-breaks type="array">
    // <tie-break>match wins vs tied</tie-break>
    // <tie-break>game wins</tie-break>
    // <tie-break>points scored</tie-break>
    // </tie-breaks>
    /// A type of the tournament
    pub tournament_type: TournamentType,

    /// Time when the tournament was updated last time
    pub updated_at: DateTime<FixedOffset>,

    /// Tournament url
    pub url: String,

    /// ???
    pub description_source: String,
    // <subdomain nil="true"/>
    /// Full url to the web page of the tournament in challonge system
    pub full_challonge_url: String,

    /// A url of `LIVE` image.
    pub live_image_url: String,
    // <sign-up-url nil="true"/>
    /// Tournament must be reviewed before finalizing.
    pub review_before_finalizing: bool,

    /// Tournament accepts predictions
    pub accepting_predictions: bool,

    /// Participants are locked: can't be added or removed
    pub participants_locked: bool,

    /// Name of the game the tournament belongs to.
    pub game_name: String,

    /// Participants can be swapped in brackets
    pub participants_swappable: bool,

    /// ???
    pub team_convertable: bool,

    /// Are the group stages were started already
    pub group_stages_were_started: bool,
}
impl Tournament {
    /// Decodes `Tournament` from JSON.
    pub fn decode(value: Value) -> Result<Tournament, Error> {
        let mut value = into_map(value)?;
        let t = remove(&mut value, "tournament")?;
        let mut tv = into_map(t)?;

        let mut started_at = None;
        if let Some(dt_str) = remove(&mut tv, "started_at")?.as_string() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(dt_str) {
                started_at = Some(dt);
            }
        }

        Ok(Tournament {
            accept_attachments: remove(&mut tv, "accept_attachments")?
                .as_boolean()
                .unwrap_or(false),
            allow_participant_match_reporting: remove(
                &mut tv,
                "allow_participant_match_reporting",
            )?
            .as_boolean()
            .unwrap_or(false),
            anonymous_voting: remove(&mut tv, "anonymous_voting")?
                .as_boolean()
                .unwrap_or(false),
            created_at: DateTime::parse_from_rfc3339(
                remove(&mut tv, "created_at")?.as_string().unwrap_or(""),
            )
            .unwrap(),
            created_by_api: remove(&mut tv, "created_by_api")?
                .as_boolean()
                .unwrap_or(false),
            credit_capped: remove(&mut tv, "credit_capped")?
                .as_boolean()
                .unwrap_or(false),
            description: remove(&mut tv, "description")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            game_id: remove(&mut tv, "game_id")?.as_u64().unwrap_or(0),
            id: TournamentId::Id(remove(&mut tv, "id")?.as_u64().unwrap_or(0)),
            name: remove(&mut tv, "name")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            group_stages_enabled: remove(&mut tv, "group_stages_enabled")?
                .as_boolean()
                .unwrap_or(false),
            hide_forum: remove(&mut tv, "hide_forum")?.as_boolean().unwrap_or(false),
            hide_seeds: remove(&mut tv, "hide_seeds")?.as_boolean().unwrap_or(false),
            hold_third_place_match: remove(&mut tv, "hold_third_place_match")?
                .as_boolean()
                .unwrap_or(false),
            max_predictions_per_user: remove(&mut tv, "max_predictions_per_user")?
                .as_u64()
                .unwrap_or(0),
            notify_users_when_matches_open: remove(&mut tv, "notify_users_when_matches_open")?
                .as_boolean()
                .unwrap_or(false),
            notify_users_when_the_tournament_ends: remove(
                &mut tv,
                "notify_users_when_the_tournament_ends",
            )?
            .as_boolean()
            .unwrap_or(false),
            open_signup: remove(&mut tv, "open_signup")?
                .as_boolean()
                .unwrap_or(false),
            participants_count: remove(&mut tv, "participants_count")?.as_u64().unwrap_or(0),
            prediction_method: remove(&mut tv, "prediction_method")?.as_u64().unwrap_or(0),
            private: remove(&mut tv, "private")?.as_boolean().unwrap_or(false),
            progress_meter: remove(&mut tv, "progress_meter")?.as_u64().unwrap_or(0),
            swiss_points: GamePoints::decode(&mut tv, "").unwrap(),
            quick_advance: remove(&mut tv, "quick_advance")?
                .as_boolean()
                .unwrap_or(false),
            require_score_agreement: remove(&mut tv, "require_score_agreement")?
                .as_boolean()
                .unwrap_or(false),
            round_robin_points: GamePoints::decode(&mut tv, "rr_").unwrap(),
            sequential_pairings: remove(&mut tv, "sequential_pairings")?
                .as_boolean()
                .unwrap_or(false),
            show_rounds: remove(&mut tv, "show_rounds")?
                .as_boolean()
                .unwrap_or(false),
            started_at,
            swiss_rounds: remove(&mut tv, "swiss_rounds")?.as_u64().unwrap_or(0),
            teams: remove(&mut tv, "teams")?.as_boolean().unwrap_or(false),
            tournament_type: TournamentType::from_str(
                remove(&mut tv, "tournament_type")?
                    .as_string()
                    .unwrap_or(""),
            )
            .unwrap_or(TournamentType::SingleElimination),
            updated_at: DateTime::parse_from_rfc3339(
                remove(&mut tv, "updated_at")?.as_string().unwrap(),
            )
            .unwrap(),
            url: remove(&mut tv, "url")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            description_source: remove(&mut tv, "description_source")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            full_challonge_url: remove(&mut tv, "full_challonge_url")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            live_image_url: remove(&mut tv, "live_image_url")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            review_before_finalizing: remove(&mut tv, "review_before_finalizing")?
                .as_boolean()
                .unwrap_or(false),
            accepting_predictions: remove(&mut tv, "accepting_predictions")?
                .as_boolean()
                .unwrap_or(false),
            participants_locked: remove(&mut tv, "participants_locked")?
                .as_boolean()
                .unwrap_or(false),
            game_name: remove(&mut tv, "game_name")?
                .as_string()
                .unwrap_or("")
                .to_string(),
            participants_swappable: remove(&mut tv, "participants_swappable")?
                .as_boolean()
                .unwrap_or(false),
            team_convertable: remove(&mut tv, "team_convertable")?
                .as_boolean()
                .unwrap_or(false),
            group_stages_were_started: remove(&mut tv, "group_stages_were_started")?
                .as_boolean()
                .unwrap_or(false),
        })
    }
}

/// A list of tournaments of the account/organization.
#[derive(Debug, Clone)]
pub struct Index(pub Vec<Tournament>);
impl Index {
    /// Decodes tournament index from JSON.
    pub fn decode(value: Value) -> Result<Index, Error> {
        Ok(Index(decode_array(value, Tournament::decode)?))
    }
}

/// A type of a tournament.
#[derive(Debug, Clone, PartialEq)]
pub enum TournamentType {
    /// [Single elimination system](https://en.wikipedia.org/wiki/Single-elimination_tournament)
    SingleElimination,

    /// [Double elimination system](https://en.wikipedia.org/wiki/Double-elimination_tournament)
    DoubleElimination,

    /// [Round robin tournament system](https://en.wikipedia.org/wiki/Round-robin_tournament)
    RoundRobin,

    /// [Swiss tournament system](https://en.wikipedia.org/wiki/Swiss-system_tournament)
    Swiss,
}
impl TournamentType {
    /// Parses tournament type to GET HTTP-method parameters string
    pub fn to_get_param<'a>(&self) -> &'a str {
        match *self {
            TournamentType::SingleElimination => "single_elimination",
            TournamentType::DoubleElimination => "double_elimination",
            TournamentType::RoundRobin => "round_robin",
            TournamentType::Swiss => "swiss",
        }
    }
}
impl fmt::Display for TournamentType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TournamentType::SingleElimination => fmt.write_str("single elimination"),
            TournamentType::DoubleElimination => fmt.write_str("double elimination"),
            TournamentType::RoundRobin => fmt.write_str("round robin"),
            TournamentType::Swiss => fmt.write_str("swiss"),
        }
    }
}
impl FromStr for TournamentType {
    type Err = ();
    fn from_str(s: &str) -> Result<TournamentType, ()> {
        match s {
            "single_elimination" => Ok(TournamentType::SingleElimination),
            "single elimination" => Ok(TournamentType::SingleElimination),
            "double_elimination" => Ok(TournamentType::DoubleElimination),
            "double elimination" => Ok(TournamentType::DoubleElimination),
            "round_robin" => Ok(TournamentType::RoundRobin),
            "round robin" => Ok(TournamentType::RoundRobin),
            "swiss" => Ok(TournamentType::Swiss),
            _ => Err(()),
        }
    }
}

/// Current tournament state.
#[derive(Debug, Clone)]
pub enum TournamentState {
    /// Tournament is in any state
    All,

    /// Tournament is in pending state
    Pending,

    /// Tournament is in progress at this moment
    InProgress,

    /// Tournament is finished
    Ended,
}
impl fmt::Display for TournamentState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TournamentState::All => {
                fmt.write_str("all")?;
            }
            TournamentState::Pending => {
                fmt.write_str("pending")?;
            }
            TournamentState::InProgress => {
                fmt.write_str("in_progress")?;
            }
            TournamentState::Ended => {
                fmt.write_str("ended")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use tournament::{Tournament, TournamentId, TournamentType};

    #[test]
    fn test_tournament_parse() {
        let string = r#"{
          "tournament": {
            "accept_attachments": false,
            "allow_participant_match_reporting": true,
            "anonymous_voting": false,
            "category": null,
            "check_in_duration": null,
            "completed_at": null,
            "created_at": "2015-01-19T16:47:30-05:00",
            "created_by_api": false,
            "credit_capped": false,
            "description": "sample description",
            "game_id": 600,
            "group_stages_enabled": false,
            "hide_forum": false,
            "hide_seeds": false,
            "hold_third_place_match": false,
            "id": 1086875,
            "max_predictions_per_user": 1,
            "name": "Sample Tournament 1",
            "notify_users_when_matches_open": true,
            "notify_users_when_the_tournament_ends": true,
            "open_signup": false,
            "participants_count": 4,
            "prediction_method": 0,
            "predictions_opened_at": null,
            "private": false,
            "progress_meter": 0,
            "pts_for_bye": "1.0",
            "pts_for_game_tie": "0.0",
            "pts_for_game_win": "0.0",
            "pts_for_match_tie": "0.5",
            "pts_for_match_win": "1.0",
            "quick_advance": false,
            "ranked_by": "match wins",
            "require_score_agreement": false,
            "rr_pts_for_game_tie": "0.0",
            "rr_pts_for_game_win": "0.0",
            "rr_pts_for_match_tie": "0.5",
            "rr_pts_for_match_win": "1.0",
            "sequential_pairings": false,
            "show_rounds": true,
            "signup_cap": null,
            "start_at": null,
            "started_at": "2015-01-19T16:57:17-05:00",
            "started_checking_in_at": null,
            "state": "underway",
            "swiss_rounds": 0,
            "teams": false,
            "tie_breaks": [
              "match wins vs tied",
              "game wins",
              "points scored"
            ],
            "tournament_type": "single elimination",
            "updated_at": "2015-01-19T16:57:17-05:00",
            "url": "sample_tournament_1",
            "description_source": "sample description source",
            "subdomain": null,
            "full_challonge_url": "http://challonge.com/sample_tournament_1",
            "live_image_url": "http://images.challonge.com/sample_tournament_1.png",
            "sign_up_url": null,
            "review_before_finalizing": true,
            "accepting_predictions": false,
            "participants_locked": true,
            "game_name": "Table Tennis",
            "participants_swappable": false,
            "team_convertable": false,
            "group_stages_were_started": false
          }
        }"#;
        let json_r = serde_json::from_str(string);
        assert!(json_r.is_ok());
        let json = json_r.unwrap();
        if let Ok(t) = Tournament::decode(json) {
            assert_eq!(t.accept_attachments, false);
            assert_eq!(t.allow_participant_match_reporting, true);
            assert_eq!(t.anonymous_voting, false);
            // assert_eq!(t.created_at, DateTime<);
            assert_eq!(t.created_by_api, false);
            assert_eq!(t.description, "sample description");
            assert_eq!(t.credit_capped, false);
            assert_eq!(t.game_id, 600);
            if let TournamentId::Id(num) = t.id {
                assert_eq!(num, 1086875);
            } else {
                // Id should be parsed as numeric variable here.
                assert!(false);
            }
            assert_eq!(t.name, "Sample Tournament 1");
            assert_eq!(t.group_stages_enabled, false);
            assert_eq!(t.hide_forum, false);
            assert_eq!(t.hide_seeds, false);
            assert_eq!(t.hold_third_place_match, false);
            assert_eq!(t.max_predictions_per_user, 1);
            assert_eq!(t.notify_users_when_matches_open, true);
            assert_eq!(t.notify_users_when_the_tournament_ends, true);
            assert_eq!(t.open_signup, false);
            assert_eq!(t.participants_count, 4);
            assert_eq!(t.prediction_method, 0);
            assert_eq!(t.private, false);
            assert_eq!(t.progress_meter, 0);
            assert_eq!(t.swiss_points.bye, Some(1.0f64));
            assert_eq!(t.swiss_points.game_tie, 0.0f64);
            assert_eq!(t.swiss_points.game_win, 0.0f64);
            assert_eq!(t.swiss_points.match_tie, 0.5f64);
            assert_eq!(t.swiss_points.match_win, 1.0f64);
            assert_eq!(t.quick_advance, false);
            assert_eq!(t.require_score_agreement, false);
            assert_eq!(t.round_robin_points.game_tie, 0.0f64);
            assert_eq!(t.round_robin_points.game_win, 0.0f64);
            assert_eq!(t.round_robin_points.match_tie, 0.5f64);
            assert_eq!(t.round_robin_points.match_win, 1.0f64);
            assert_eq!(t.sequential_pairings, false);
            assert_eq!(t.show_rounds, true);
            // assert_eq!(t.started_at, DateTime<);
            assert_eq!(t.swiss_rounds, 0);
            assert_eq!(t.teams, false);
            assert_eq!(t.tournament_type, TournamentType::SingleElimination);
            // assert_eq!(t.updated_at, DateTime<);
            assert_eq!(t.url, "sample_tournament_1");
            assert_eq!(t.description_source, "sample description source");
            assert_eq!(
                t.full_challonge_url,
                "http://challonge.com/sample_tournament_1"
            );
            assert_eq!(
                t.live_image_url,
                "http://images.challonge.com/sample_tournament_1.png"
            );
            assert_eq!(t.review_before_finalizing, true);
            assert_eq!(t.accepting_predictions, false);
            assert_eq!(t.participants_locked, true);
            assert_eq!(t.game_name, "Table Tennis");
            assert_eq!(t.participants_swappable, false);
            assert_eq!(t.team_convertable, false);
            assert_eq!(t.group_stages_were_started, false);
        } else {
            assert!(false);
        }
    }
}

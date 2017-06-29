//! Challonge Tournament type.

use std::fmt;

use chrono::*;
use serde;


use error::Error;


/// Tournament includes.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TournamentIncludes {
    /// Includes matches and participants
    All,
    /// Includes matches
    Matches,
    /// Includes participants
    Participants,
}

/// Tournament ranking order.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum RankedBy {
    /// Rank by number of matches won
    #[serde(rename = "match wins")]
    MatchWins,
    /// Rank by number of games won
    #[serde(rename = "game wins")]
    GameWins,
    /// Rank by points scored
    #[serde(rename = "points scored")]
    PointsScored,
    /// Rank by difference in points
    #[serde(rename = "points difference")]
    PointsDifference,
    /// Custom ranking rules
    #[serde(rename = "custom")]
    Custom,
}


/// A type of a tournament.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum TournamentType {
    /// [Single elimination system](https://en.wikipedia.org/wiki/Single-elimination_tournament)
    #[serde(rename = "single elimination")]
    SingleElimination,
    /// [Double elimination system](https://en.wikipedia.org/wiki/Double-elimination_tournament)
    #[serde(rename = "double elimination")]
    DoubleElimination,
    /// [Round robin tournament system](https://en.wikipedia.org/wiki/Round-robin_tournament)
    #[serde(rename = "round robin")]
    RoundRobin,
    /// [Swiss tournament system](https://en.wikipedia.org/wiki/Swiss-system_tournament)
    #[serde(rename = "swiss")]
    Swiss
}


/// Describes a tournament state.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TournamentState {
    /// Tournament is in any state
    #[serde(rename = "all")]
    All,
    /// Tournament is in pending state
    #[serde(rename = "pending")]
    Pending,
    /// Tournament is in progress at this moment
    #[serde(rename = "in_progress")]
    InProgress,
    /// Tournament is finished
    #[serde(rename = "ended")]
    Ended
}


/// Tournament ID is an integer value or pair of strings (subdomain and tournament url)
#[derive(Debug, Clone, PartialEq)]
pub enum TournamentId {
    /// Subdomain and Tournament url
    Url(String, String),

    /// Unique idenfifier (number) in challonge system
    Id(u64)
}
impl serde::Serialize for TournamentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        match *self {
            TournamentId::Url(ref subdomain, ref url) => {
                serializer.serialize_str(&format!("{}-{}", subdomain, url))
            },
            TournamentId::Id(id) => serializer.serialize_u64(id),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TournamentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TournamentId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("positive integer")
            }

            fn visit_u64<E>(self, value: u64) -> Result<TournamentId, E>
                where E: serde::de::Error {
                Ok(TournamentId::Id(value))
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_u64(Visitor)
    }
}
impl fmt::Display for TournamentId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TournamentId::Url(ref subdomain, ref tournament_url) => {
                try!(fmt.write_str(&format!("{}-{}", subdomain, tournament_url)));
            },
            TournamentId::Id(ref id) => {
                try!(fmt.write_str(&id.to_string()));
            },
        }
        Ok(())
    }
}

/// Game points definition for swiss system.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwissGamePoints {
    /// Points for winning a match
    #[serde(rename = "pts_for_match_win")]
    pub match_win: f64,
    /// Points for tie match
    #[serde(rename = "pts_for_match_tie")]
    pub match_tie: f64,
    /// Points for winning a game
    #[serde(rename = "pts_for_game_win")]
    pub game_win: f64,
    /// Points for a tie game
    #[serde(rename = "pts_for_game_tie")]
    pub game_tie: f64,
    /// ??? Points for leaving the tournament ???
    #[serde(rename = "pts_for_bye")]
    pub bye: Option<f64>,
}
impl Default for SwissGamePoints {
    fn default() -> SwissGamePoints {
        SwissGamePoints {
            match_win: 0.5f64,
            match_tie: 1.0f64,
            game_win: 0.0f64,
            game_tie: 0.0f64,
            bye: None,
        }
    }
}

/// Game points definition for round-robin system.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundRobinGamePoints {
    /// Points for winning a match
    #[serde(rename = "rr_pts_for_match_win")]
    pub match_win: f64,
    /// Points for tie match
    #[serde(rename = "rr_pts_for_match_tie")]
    pub match_tie: f64,
    /// Points for winning a game
    #[serde(rename = "rr_pts_for_game_win")]
    pub game_win: f64,
    /// Points for a tie game
    #[serde(rename = "rr_pts_for_game_tie")]
    pub game_tie: f64,
    /// ??? Points for leaving the tournament ???
    #[serde(rename = "rr_pts_for_bye")]
    pub bye: Option<f64>,
}
impl Default for RoundRobinGamePoints {
    fn default() -> RoundRobinGamePoints {
        RoundRobinGamePoints {
            match_win: 0.5f64,
            match_tie: 1.0f64,
            game_win: 0.0f64,
            game_tie: 0.0f64,
            bye: None,
        }
    }
}

/// A separate structure for creating a tournament.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /* https://github.com/serde-rs/serde/issues/969
    /// Only for Swiss system
    pub swiss_points: SwissGamePoints,
    */
    /// Number of rounds in swiss system
    pub swiss_rounds: u64,
    /// Tournament ranking type
    pub ranked_by: RankedBy,
    /* https://github.com/serde-rs/serde/issues/969
    /// Only for Round Robin system
    // pub round_robin_points: RoundRobinGamePoints,
    */
    /// Single &amp; Double Elimination only - Label each round above the bracket (default: false)
    pub show_rounds: bool,
    /// Hide this tournament from the public browsable index and your profile (default: false)
    #[serde(rename = "private")]
    pub is_private: bool,
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
            // swiss_points: GamePoints::new(0.5f64, 1.0f64, 0.0f64, 0.0f64, Some(0.0f64)),
            swiss_rounds: 0,
            ranked_by: RankedBy::PointsScored,
            // round_robin_points: GamePoints::default(),
            show_rounds: false,
            is_private: false,
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
    // builder!(swiss_points, GamePoints);
    builder!(swiss_rounds, u64);
    builder!(ranked_by, RankedBy);
    // builder!(round_robin_points, GamePoints);
    builder!(show_rounds, bool);
    builder!(is_private, bool);
    builder_so!(game_name);
    builder!(notify_users_when_matches_open, bool);
    builder!(notify_users_when_the_tournament_ends, bool);
    builder!(sequential_pairings, bool);
    builder!(signup_cap, u64);
    builder!(check_in_duration, u64);
    builder!(grand_finals_modifier, Option<String>);
}

/// Challonge `Tournament` definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /*
    /// A points for matches/games in swiss system
    pub swiss_points: SwissGamePoints,
    */
    /// ???
    pub quick_advance: bool,
    // <ranked-by>match wins</ranked-by>
    /// Tournament will require score agreement from all of participants of the match
    pub require_score_agreement: bool,
    /*
    /// A points for matches/games in round robin system
    pub round_robin_points: RoundRobinGamePoints,
    */
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

/// A list of tournaments of the account/organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index(pub Vec<Tournament>);

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use tournament::{ Tournament, TournamentType, TournamentId };

    #[test]
    fn test_tournament_parse() {
        let string = r#"{
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
        }"#;
        let t: Tournament = serde_json::from_str(string).unwrap();
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
        // assert_eq!(t.swiss_points.bye, Some(1.0f64));
        // assert_eq!(t.swiss_points.game_tie, 0.0f64);
        // assert_eq!(t.swiss_points.game_win, 0.0f64);
        // assert_eq!(t.swiss_points.match_tie, 0.5f64);
        // assert_eq!(t.swiss_points.match_win, 1.0f64);
        assert_eq!(t.quick_advance, false);
        assert_eq!(t.require_score_agreement, false);
        // assert_eq!(t.round_robin_points.game_tie, 0.0f64);
        // assert_eq!(t.round_robin_points.game_win, 0.0f64);
        // assert_eq!(t.round_robin_points.match_tie, 0.5f64);
        // assert_eq!(t.round_robin_points.match_win, 1.0f64);
        assert_eq!(t.sequential_pairings, false);
        assert_eq!(t.show_rounds, true);
        // assert_eq!(t.started_at, DateTime<);
        assert_eq!(t.swiss_rounds, 0);
        assert_eq!(t.teams, false);
        assert_eq!(t.tournament_type, TournamentType::SingleElimination);
        // assert_eq!(t.updated_at, DateTime<);
        assert_eq!(t.url, "sample_tournament_1");
        assert_eq!(t.description_source, "sample description source");
        assert_eq!(t.full_challonge_url, "http://challonge.com/sample_tournament_1");
        assert_eq!(t.live_image_url, "http://images.challonge.com/sample_tournament_1.png");
        assert_eq!(t.review_before_finalizing, true);
        assert_eq!(t.accepting_predictions, false);
        assert_eq!(t.participants_locked, true);
        assert_eq!(t.game_name, "Table Tennis");
        assert_eq!(t.participants_swappable, false);
        assert_eq!(t.team_convertable, false);
        assert_eq!(t.group_stages_were_started, false);
    }
}

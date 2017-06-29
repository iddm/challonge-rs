//! Challonge Match type.

use chrono::*;

use std::fmt;
use std::str::FromStr;

use error::Error;
use participants::ParticipantId;
use tournament::TournamentId;


/// Represents a pair of scores - for player 1 and player 2 respectively.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchScore(pub u64, pub u64);
impl MatchScore {
    /// Decodes `MatchScore` from JSON.
    pub fn decode(string: &str) -> Result<MatchScore, Error> {
        let mut parts = string.trim().split('-');
        Ok(MatchScore (
            parts.next().unwrap_or("").trim().parse::<u64>().unwrap_or(0),
            parts.next().unwrap_or("").trim().parse::<u64>().unwrap_or(0),
        ))
    }
}
impl fmt::Display for MatchScore {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str(&format!("{}-{}", self.0, self.1)));
        Ok(())
    }
}

/// A list of scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchScores(pub Vec<MatchScore>);

/// Represents an ID of a match
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchId(pub u64);

/// Current match state.
#[derive(Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MatchState {
    /// Any state of a match.
    All,
    /// Match is in a pending state.
    Pending,
    /// Match is open.
    Open,
    /// Match is completed.
    Complete,
}

/// A list of matches of the tournament.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index(pub Vec<Match>);


/// NOTE: If you're updating winner_id, scores_csv must also be provided. You may, however, update
/// score_csv without providing winner_id for live score updates.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MatchUpdate {
    /// Comma separated set/game scores with player 1 score first (e.g. "1-3,3-0,3-2")
    pub scores_csv: MatchScores,
    /// The participant ID of the winner or "tie" if applicable (Round Robin and Swiss).
    /// NOTE: If you change the outcome of a completed match, all matches in the bracket that branch from the updated match will be reset.
    pub winner_id: Option<ParticipantId>,
    /// Overwrites the number of votes for player 1
    pub player1_votes: Option<u64>,
    /// Overwrites the number of votes for player 2
    pub player2_votes: Option<u64>,
}
impl MatchUpdate {
    builder!(scores_csv, MatchScores);
    builder_o!(winner_id, ParticipantId);
    builder_o!(player1_votes, u64);
    builder_o!(player2_votes, u64);
}

/// Player data in match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Unique participant identifier
    pub id: ParticipantId,
    /// ???
    pub is_prereq_match_loser: bool,
    /// ???
    pub prereq_match_id: Option<MatchId>,
    /// Number of votes to the user.
    pub votes: u64
}
impl Player {
    /// Decodes `Player` from JSON
    pub fn decode(mut map: &mut BTreeMap<String, Value>, prefix: &str) -> Result<Player, Error> {
        Ok(Player {
            id: ParticipantId(try!(remove(&mut map, &format!("{}id", prefix))).as_u64().unwrap_or(0)),
            is_prereq_match_loser: try!(remove(&mut map, &format!("{}is_prereq_match_loser", prefix))).as_boolean().unwrap_or(false),
            prereq_match_id: try!(remove(&mut map, &format!("{}prereq_match_id", prefix)))
                .as_u64().map_or(None, |i| Some(MatchId(i))),
            votes: try!(remove(&mut map, &format!("{}votes", prefix))).as_u64().unwrap_or(0),
        })
    }
}

/// Challonge `Match` definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    // attachment_count: ,
    /// Holds a time when match was created.
    pub created_at: DateTime<FixedOffset>,
    // group_id: ,
    /// Does the match has an attachment?
    pub has_attachment: bool,
    /// Unique Match identifier
    pub id: MatchId,
    /// ???
    pub identifier: String,
    // location:
    /// An id of user which lost the match
    pub loser_id: Option<ParticipantId>,
    /// Information about first player
    pub player1: Player,
    /// Information about second player
    pub player2: Player,
    /// Number of current round of the match.
    pub round: u64,
    // // // scheduled_time:
    /// Holds a time when match was started.
    pub started_at: Option<DateTime<FixedOffset>>,
    /// State of the match.
    pub state: MatchState,
    /// Id of a tournament to which this match belongs.
    pub tournament_id: TournamentId,
    // // underway_at:
    /// A time when match was updated last time.
    pub updated_at: DateTime<FixedOffset>,
    /// An id of user which won the match
    pub winner_id: Option<ParticipantId>,
    /// ???
    pub prerequisite_match_ids_csv: String,
    /// Match scores (pairs of score for first and second player)
    pub scores_csv: MatchScores,
}
impl Match {
    /// Decodes `Match` from JSON
    pub fn decode(value: Value) -> Result<Match, Error> {
        let mut value = try!(into_map(value));
        let t = try!(remove(&mut value, "match"));
        let mut tv = try!(into_map(t));

        let mut started_at = None;
        if let Some(sa_str) = try!(remove(&mut tv, "started_at")).as_string() {
            if let Ok(sa) = DateTime::parse_from_rfc3339(sa_str) {
                started_at = Some(sa);
            }
        }

        Ok(Match {
            created_at: DateTime::parse_from_rfc3339(try!(remove(&mut tv, "created_at")).as_string().unwrap_or("")).unwrap(),
            has_attachment: try!(remove(&mut tv, "has_attachment")).as_boolean().unwrap_or(false),
            id: MatchId(try!(remove(&mut tv, "id")).as_u64().unwrap()),
            identifier: try!(remove(&mut tv, "identifier")).as_string().unwrap_or("").to_owned(),
            loser_id: try!(remove(&mut tv, "loser_id")).as_u64()
                .map_or(None, |i| Some(ParticipantId(i))),
            player1: Player::decode(&mut tv, "player1_").unwrap(),
            player2: Player::decode(&mut tv, "player2_").unwrap(),
            round: try!(remove(&mut tv, "round")).as_u64().unwrap(),
            started_at: started_at,
            state: MatchState::from_str(try!(remove(&mut tv, "state")).as_string()
                                        .unwrap_or(""))
                                        .unwrap_or(MatchState::All),
            tournament_id: TournamentId::Id(try!(remove(&mut tv, "tournament_id")).as_u64().unwrap()),
            updated_at: DateTime::parse_from_rfc3339(try!(remove(&mut tv, "updated_at")).as_string().unwrap_or("")).unwrap(),
            winner_id: try!(remove(&mut tv, "winner_id")).as_u64()
                .map_or(None, |i| Some(ParticipantId(i))),
            prerequisite_match_ids_csv: try!(remove(&mut tv, "prerequisite_match_ids_csv")).as_string().unwrap_or("").to_owned(),
            scores_csv: MatchScores::decode(try!(remove(&mut tv, "scores_csv")).as_string().unwrap_or("").to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use matches::{
        Match,
        MatchState,
        MatchScore,
    };
    use tournament::TournamentId;


    #[test]
    fn test_score_parse() {
        let strings = vec!["3-1", "", "3-0", "3--5", "0-0", "  9-", "    -    118  "];
        let correct_scores = vec![
            MatchScore(3, 1),
            MatchScore(0, 0),
            MatchScore(3, 0),
            MatchScore(3, 0),
            MatchScore(0, 0),
            MatchScore(9, 0),
            MatchScore(0, 118),
        ];
        let mut iter = strings.iter().zip(correct_scores.iter());
        while let Some(pair) = iter.next() {
            if let Ok(ms) = MatchScore::decode(pair.0) {
                assert_eq!(ms.0, (pair.1).0);
                assert_eq!(ms.1, (pair.1).1);
                assert_eq!(ms.to_string(), (pair.1).to_string());
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_participant_parse() {
        let string = r#"{
          "match": {
            "attachment_count": null,
            "created_at": "2015-01-19T16:57:17-05:00",
            "group_id": null,
            "has_attachment": false,
            "id": 23575258,
            "identifier": "A",
            "location": null,
            "loser_id": null,
            "player1_id": 16543993,
            "player1_is_prereq_match_loser": false,
            "player1_prereq_match_id": null,
            "player1_votes": null,
            "player2_id": 16543997,
            "player2_is_prereq_match_loser": false,
            "player2_prereq_match_id": null,
            "player2_votes": 3,
            "round": 1,
            "scheduled_time": null,
            "started_at": "2015-01-19T16:57:17-05:00",
            "state": "open",
            "tournament_id": 1086875,
            "underway_at": null,
            "updated_at": "2015-01-19T16:57:17-05:00",
            "winner_id": null,
            "prerequisite_match_ids_csv": "",
            "scores_csv": "3-1, 3-2"
          }
        }"#;
        let json_r = serde_json::from_str(string);
        assert!(json_r.is_ok());
        let json = json_r.unwrap();
        if let Ok(m) = Match::decode(json) {
            // assert_eq!(m.attachment_count, );
            // assert_eq!(m.created_at, );
            // assert_eq!(m.group_id, );
            assert_eq!(m.has_attachment, false);
            assert_eq!(m.id.0, 23575258);
            assert_eq!(m.identifier, "A");
            assert_eq!(m.loser_id, None);
            assert_eq!(m.player1.id.0, 16543993);
            assert_eq!(m.player1.is_prereq_match_loser, false);
            assert_eq!(m.player1.prereq_match_id, None);
            assert_eq!(m.player1.votes, 0);
            assert_eq!(m.player2.is_prereq_match_loser, false);
            assert_eq!(m.player2.prereq_match_id, None);
            assert_eq!(m.player2.id.0, 16543997);
            assert_eq!(m.player2.votes, 3);
            assert_eq!(m.round, 1);
            // assert_eq!(m.started_at, );
            assert_eq!(m.state, MatchState::Open);
            assert_eq!(m.tournament_id, TournamentId::Id(1086875));
            // assert_eq!(m.updated_at, );
            assert_eq!(m.winner_id, None);
            assert!(m.prerequisite_match_ids_csv.is_empty());
            {
                let correct_scores = vec![
                    MatchScore(3, 1),
                    MatchScore(3, 2),
                ];
                assert_eq!(m.scores_csv.0.len(), 2);
                let mut iter = m.scores_csv.0.iter().zip(correct_scores.iter());
                while let Some(pair) = iter.next() {
                    assert_eq!((pair.0).0, (pair.1).0);
                    assert_eq!((pair.1).1, (pair.1).1);
                }
            }
        } else {
            assert!(false);
        }
    }
}

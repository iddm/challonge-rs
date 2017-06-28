//! Client library for the [Challonge](https://challonge.com) REST API.
//!
//! Log in to Challonge with `Challonge::new`.
//! Call API methods to interact with the service.
//!
//! For Challonge API documentation [look here](http://api.challonge.com/ru/v1/documents).
//!
//! For examples, see the `examples` directory in the source tree.
#![warn(missing_docs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate reqwest;

use chrono::Date;
use chrono::offset::Local;
use reqwest::header::{ Basic, Authorization };


#[macro_use]
mod macroses;
mod endpoints;
mod tournament;
// mod participants;
// mod matches;
// mod attachments;
pub mod error;
pub use tournament::{
//     Tournament,
    TournamentId,
//     TournamentCreate,
    // TournamentState,
    // TournamentType,
    TournamentIncludes,
    // Index as TournamentIndex,
};
// pub use participants::{
//     Participant,
//     ParticipantCreate,
//     ParticipantId,
//     Index as ParticipantIndex,
// };
// pub use matches::{
//     Match,
//     MatchScore,
//     MatchScores,
//     MatchState,
//     MatchUpdate,
//     MatchId,
//     Index as MatchIndex,
// };
// pub use attachments::{
//     AttachmentId,
//     Attachment,
//     AttachmentCreate,
//     Index as AttachmentIndex,
// };
use endpoints::Endpoint;
use error::{ Error, Result };


/// Macro only for internal use with the `Challonge` object (relies on it's fields)
macro_rules! request {
    ($api: ident, $method: ident, $address: expr) => {
        {
            retry(|| $api.client.$method($address)
                                .header(Authorization(Basic {
                                    username: $api.username,
                                    password: Some($api.password),
                                })))
        }
    }
}

/// Macro only for internal use with the `Challonge` object (relies on it's fields)
macro_rules! request_body {
    ($api: ident, $method: ident, $address: expr, $body: expr) => {
        {
            retry(|| $api.client.$method($address)
                                .body($body)
                                .header(Authorization(Basic {
                                    username: $api.username,
                                    password: Some($api.password),
                                })))
        }
    };
}


fn check_status(response: reqwest::Result<reqwest::Response>)
    -> Result<reqwest::Response> {
    let response = response?;
    if !response.status().is_success() {
        return Err(Error::from(response))
    }
    Ok(response)
}

fn retry<F: Fn() -> reqwest::RequestBuilder>(f: F) -> Result<reqwest::Response> {
    let f2 = || check_status(f().send());
    // retry on a ConnectionAborted, which occurs if it's been a while since the last request
    match f2() {
        Err(_) => f2(),
        other => other
    }
}

// type FieldPairs = Vec<(&'static str, String)>;
//
// fn pairs_to_string(params: FieldPairs) -> String {
//     let mut body = String::new();
//     let mut sep = "";
//     for p in params {
//         body.push_str(sep);
//         body.push_str(&format!("{}={}", p.0, p.1));
//         sep = "&";
//     }
//     body
// }
//
// fn pcs_to_pairs(participants: Vec<ParticipantCreate>) -> FieldPairs {
//     let mut params = Vec::new();
//     for p in participants {
//         params.push((ps!("email"), p.email.clone()));
//         params.push((ps!("seed"), p.seed.to_string()));
//         params.push((ps!("misc"), p.misc.clone()));
//
//         if let Some(n) = p.name.as_ref() {
//             params.push((ps!("name"), n.clone()));
//         }
//         if let Some(un) = p.challonge_username.as_ref() {
//             params.push((ps!("challonge_username"), un.clone()));
//         }
//     }
//     params
// }
//
// fn pc_to_pairs(participant: &ParticipantCreate) -> FieldPairs {
//     let mut params = vec![
//         (p!("email"), participant.email.clone()),
//         (p!("seed"), participant.seed.to_string()),
//         (p!("misc"), participant.misc.clone()),
//     ];
//
//     if let Some(n) = participant.name.as_ref() {
//         params.push((p!("name"), n.clone()));
//     }
//     if let Some(un) = participant.challonge_username.as_ref() {
//         params.push((p!("challonge_username"), un.clone()));
//     }
//     params
// }
//
// fn at_to_pairs(attachment: &AttachmentCreate) -> FieldPairs {
//     let mut params = FieldPairs::new();
//
//     if let Some(a) = attachment.asset.as_ref() {
//         params.push((a!("asset"), String::from_utf8(a.clone()).unwrap()));
//     }
//     if let Some(url) = attachment.url.as_ref() {
//         params.push((a!("url"), url.clone()));
//     }
//     if let Some(d) = attachment.description.as_ref() {
//         params.push((a!("description"), d.clone()));
//     }
//     params
// }
//
// fn tc_to_pairs(tournament: &TournamentCreate) -> FieldPairs {
//     let mut params = vec![
//         (t!("name"), tournament.name.clone()),
//         (t!("tournament_type"), tournament.tournament_type.to_string()),
//         (t!("url"), tournament.url.clone()),
//         (t!("subdomain"), tournament.subdomain.clone()),
//         (t!("description"), tournament.description.clone()),
//         (t!("open_signup"), tournament.open_signup.to_string()),
//         (t!("hold_third_place_match"), tournament.hold_third_place_match.to_string()),
//         (t!("pts_for_match_win"), tournament.swiss_points.match_win.to_string()),
//         (t!("pts_for_match_tie"), tournament.swiss_points.match_tie.to_string()),
//         (t!("pts_for_game_win"), tournament.swiss_points.game_win.to_string()),
//         (t!("pts_for_game_tie"), tournament.swiss_points.game_tie.to_string()),
//         (t!("swiss_rounds"), tournament.swiss_rounds.to_string()),
//         (t!("ranked_by"), tournament.ranked_by.to_string()),
//         (t!("rr_pts_for_match_win"), tournament.round_robin_points.match_win.to_string()),
//         (t!("rr_pts_for_match_tie"), tournament.round_robin_points.match_tie.to_string()),
//         (t!("rr_pts_for_game_win"), tournament.round_robin_points.game_win.to_string()),
//         (t!("rr_pts_for_game_tie"), tournament.round_robin_points.game_tie.to_string()),
//         (t!("show_rounds"), tournament.show_rounds.to_string()),
//         (t!("private"), tournament.private.to_string()),
//         (t!("notify_users_when_matches_open"), tournament.notify_users_when_matches_open.to_string()),
//         (t!("notify_users_when_the_tournament_ends"), tournament.notify_users_when_the_tournament_ends.to_string()),
//         (t!("sequential_pairings"), tournament.sequential_pairings.to_string()),
//         (t!("signup_cap"), tournament.signup_cap.to_string()),
//         (t!("check_in_duration"), tournament.check_in_duration.to_string()),
//     ];
//     if let Some(gfm) = tournament.grand_finals_modifier.as_ref() {
//         params.push((t!("grand_finals_modifier"), gfm.clone()));
//     }
//     if let Some(start_at) = tournament.start_at.as_ref() {
//         params.push((t!("start_at"), start_at.to_rfc3339()));
//     }
//     if let Some(s_bye_pts) = tournament.swiss_points.bye.as_ref() {
//         params.push((t!("pts_for_bye"), s_bye_pts.to_string()));
//     }
//     if let Some(game) = tournament.game_name.as_ref() {
//         params.push((t!("game_name"), game.clone()));
//     }
//     params
// }
//
// fn mu_to_pairs(mu: &MatchUpdate) -> FieldPairs {
//     let mut params = Vec::new();
//
//     if let Some(v) = mu.player1_votes {
//         params.push((m!("player1_votes"), v.to_string()));
//     }
//     if let Some(v) = mu.player2_votes {
//         params.push((m!("player2_votes"), v.to_string()));
//     }
//     params.push((m!("scores_csv"), mu.scores_csv.to_string()));
//     if let Some(w) = mu.winner_id.as_ref() {
//         params.push((m!("winner_id"), w.0.to_string()));
//     }
//     params
// }

/// Client for the Challonge REST API.
pub struct Challonge {
    user_name: String,
    password: String,
    client: reqwest::Client,
}
impl Challonge {
    /// Create new connection to Challonge.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate challonge;
    ///
    /// fn main() {
    ///     use challonge::Challonge;
    ///
    ///     let c = Challonge::new("myusername", "myapikey").unwrap();
    /// }
    /// ```
    pub fn new<S: Into<String>>(user_name: S, api_key: S) -> Result<Challonge> {
        Ok(Challonge {
            client: reqwest::Client::new()?,
            user_name: user_name.into(),
            password: api_key.into(),
        })
    }

    /*
    /// Retrieve a set of tournaments created with your account.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate challonge;
    /// extern crate chrono;
    ///
    /// fn main() {
    ///     use challonge::Challonge;
    ///     use challonge::tournament::{ TournamentState, TournamentType };
    ///     use chrono::*;
    ///
    ///     let c = Challonge::new("myusername", "myapikey").unwrap();
    ///     let index = c.tournament_index (
    ///            &TournamentState::All,
    ///            &TournamentType::DoubleElimination,
    ///            &Local::today(),
    ///            &Local::today(),
    ///            "subdomain"
    ///     );
    /// }
    /// ```
    // pub fn tournament_index(&self,
    //                         state: TournamentState,
    //                         tournament_type: TournamentType,
    //                         created_after: &Date<Local>,
    //                         created_before: &Date<Local>,
    //                         subdomain: &str) -> Result<TournamentIndex> {
        // let mut url = url!("{}/tournaments.json");
        // url.query_pairs_mut()
        //     .append_pair("state", &state.to_string())
        //     .append_pair("type", &tournament_type.to_get_param())
        //     .append_pair("created_after", &format_date!(created_after))
        //     .append_pair("created_before", &format_date!(created_before))
        //     .append_pair("subdomain", subdomain);
        //
        // let response = request!(
        // let response = try!(retry(|| self.client.get(url.as_str()).headers(self.headers.clone())));
        // TournamentIndex::decode(try!(serde_json::from_reader(response)))
    // }

    /// Retrieve a single tournament record created with your account.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate challonge;
    ///
    /// fn main() {
    ///     use challonge::Challonge;
    ///
    ///     let c = Challonge::new("myusername", "myapikey").unwrap();
    ///     let i = TournamentIncludes::Matches;
    ///     let t = c.get_tournament(&TournamentId::Id(2669881), &i);
    /// }
    /// ```
    pub fn get_tournament(&self,
                          id: &TournamentId,
                          includes: TournamentIncludes) -> Result<Tournament> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments/{}.json", API_BASE, id.to_string())).unwrap();

        Challonge::add_tournament_includes(&mut url, includes);
        let response = try!(retry(|| self.client.get(url.as_str())
                                        .headers(self.headers.clone())));
        Tournament::decode(try!(serde_json::from_reader(response)))
    }

    /// Create a new tournament.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate challonge;
    ///
    /// fn main() {
    ///     use challonge::Challonge;
    ///     use challonge::tournament::TournamentCreate;
    ///
    ///     let c = Challonge::new("myusername", "myapikey");
    ///     let tc = TournamentCreate { // explicitly define the whole structure
    ///            name: "Tester".to_owned(),
    ///            tournament_type: TournamentType::SingleElimination,
    ///            url: "testerurl".to_owned(),
    ///            subdomain: "subdomain".to_owned(),
    ///            description: "Test tournament created from challonge-rs".to_owned(),
    ///            open_signup: false,
    ///            hold_third_place_match: false,
    ///            pts_for_match_win: 0.0f64,
    ///            pts_for_match_tie: 0.0f64,
    ///            pts_for_game_win: 0.0f64,
    ///            pts_for_game_tie: 0.0f64,
    ///            pts_for_bye: 0.0f64,
    ///            swiss_rounds: 0,
    ///            ranked_by: RankedBy::PointsScored,
    ///            rr_pts_for_match_win: 0.0f64,
    ///            rr_pts_for_match_tie: 0.0f64,
    ///            rr_pts_for_game_win: 0.0f64,
    ///            rr_pts_for_game_tie: 0.0f64,
    ///            show_rounds: false,
    ///            private: false,
    ///            notify_users_when_matches_open: true,
    ///            notify_users_when_the_tournament_ends: true,
    ///            sequential_pairings: false,
    ///            signup_cap: 4,
    ///            start_at: UTC::now().add(Duration::weeks(2)),
    ///            check_in_duration: 60,
    ///            grand_finals_modifier: None,
    ///     };
    ///     let t = c.create_tournament(&tc);
    ///     // or you may create `TournamentCreate` by using a builder:
    ///     let mut tcb = TournamentCreate::new();
    ///     tcb.name("Test tournament")
    ///        .tournament_type(TournamentType::SingleElimination)
    ///        .url("TestUrl")
    ///        .subdomain("subdomain")
    ///        .description("TEST TOURNAMENT created by challonge-rs");
    ///     let tb = c.create_tournament(&tcb);
    /// }
    /// ```
    pub fn create_tournament(&self, tournament: &TournamentCreate) -> Result<Tournament> {
        let url = &format!("{}/tournaments.json", API_BASE);
        let body = pairs_to_string(tc_to_pairs(tournament));
        let response = try!(retry(|| self.client.post(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        Tournament::decode(try!(serde_json::from_reader(response)))
    }

    /// Update a tournament's attributes.
    pub fn update_tournament(&self,
                             id: &TournamentId,
                             tournament: &TournamentCreate) -> Result<Tournament> {
        let url = &format!("{}/tournaments/{}.json", API_BASE, id.to_string());
        let body = pairs_to_string(tc_to_pairs(tournament));
        let response = try!(retry(|| self.client.put(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        Tournament::decode(try!(serde_json::from_reader(response)))
    }

    /// Deletes a tournament along with all its associated records. There is no undo, so use with care!
    pub fn delete_tournament(&self, id: &TournamentId) -> Result<()> {
        let url = &format!("{}/tournaments/{}.json", API_BASE, id.to_string());
        let _ = try!(retry(|| self.client.delete(url).headers(self.headers.clone())));
        Ok(())
    }

    /// This should be invoked after a tournament's check-in window closes before the tournament is started.
    ///
    /// 1. Marks participants who have not checked in as inactive.
    /// 2. Moves inactive participants to bottom seeds (ordered by original seed).
    /// 3. Transitions the tournament state from 'checking_in' to 'checked_in'
    ///
    /// * NOTE: Checked in participants on the waiting list will be promoted if slots become available.
    pub fn tournament_process_checkins(&self,
                                       id: &TournamentId,
                                       includes: &TournamentIncludes) -> Result<()> {
        self.tournament_action("process_check_ins", id, includes)
    }

    /// When your tournament is in a 'checking_in' or 'checked_in' state, there's no way to edit the tournament's start time (start_at) or check-in duration (check_in_duration). You must first abort check-in, then you may edit those attributes.
    ///
    /// 1. Makes all participants active and clears their checked_in_at times.
    /// 2. Transitions the tournament state from 'checking_in' or 'checked_in' to 'pending'
    pub fn tournament_abort_checkins(&self,
                                     id: &TournamentId,
                                     includes: &TournamentIncludes) -> Result<()> {
        self.tournament_action("abort_check_in", id, includes)
    }

    /// Start a tournament, opening up first round matches for score reporting. The tournament must have at least 2 participants.
    pub fn tournament_start(&self,
                            id: &TournamentId,
                            includes: &TournamentIncludes) -> Result<()> {
        self.tournament_action("start", id, includes)
    }

    /// Finalize a tournament that has had all match scores submitted, rendering its results permanent.
    pub fn tournament_finalize(&self,
                               id: &TournamentId,
                               includes: &TournamentIncludes) -> Result<()> {
        self.tournament_action("finalize", id, includes)
    }

    /// Reset a tournament, clearing all of its scores and attachments. You can then add/remove/edit participants before starting the tournament again.
    pub fn tournament_reset(&self,
                            id: &TournamentId,
                            includes: &TournamentIncludes) -> Result<()> {
        self.tournament_action("reset", id, includes)
    }

    /// Retrieve a tournament's participant list.
    pub fn participant_index(&self,
                             id: &TournamentId) -> Result<ParticipantIndex> {
        let url = &format!("{}/tournaments/{}/participants.json",
                           API_BASE,
                           id.to_string());
        let response = try!(retry(|| self.client.get(url)
                                                .headers(self.headers.clone())));
        ParticipantIndex::decode(try!(serde_json::from_reader(response)))
    }

    /// Add a participant to a tournament (up until it is started).
    pub fn create_participant(&self,
                              id: &TournamentId,
                              participant: &ParticipantCreate) -> Result<Participant> {
        let url = &format!("{}/tournaments/{}/participants.json",
                           API_BASE,
                           id.to_string());
        let body = pairs_to_string(pc_to_pairs(participant));
        let response = try!(retry(|| self.client.post(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        Participant::decode(try!(serde_json::from_reader(response)))
    }

    /// Bulk add participants to a tournament (up until it is started).
    /// If an invalid participant is detected, bulk participant creation will halt and any previously added participants (from this API request) will be rolled back.
    pub fn create_participant_bulk(&self,
                                   id: &TournamentId,
                                   participants: Vec<ParticipantCreate>) -> Result<()> {
        let url = &format!("{}/tournaments/{}/participants/bulk_add.json",
                           API_BASE,
                           id.to_string());
        let body = pairs_to_string(pcs_to_pairs(participants));
        let response = try!(retry(|| self.client.post(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        let _: () = try!(serde_json::from_reader(response));
        Ok(())
    }

    /// Retrieve a single participant record for a tournament.
    pub fn get_participant(&self,
                           id: &TournamentId,
                           participant_id: &ParticipantId,
                           include_matches: bool) -> Result<Participant> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments/{}/participants/{}.json",
                                                 API_BASE,
                                                 id.to_string(),
                                                 participant_id.0)).unwrap();
        url.query_pairs_mut()
            .append_pair("include_matches", &(include_matches as i64).to_string());

        let response = try!(retry(|| self.client.get(url.as_str())
                                                .headers(self.headers.clone())));
        Participant::decode(try!(serde_json::from_reader(response)))
    }

    /// Update the attributes of a tournament participant.
    pub fn update_participant(&self,
                              id: &TournamentId,
                              participant_id: &ParticipantId,
                              participant: &ParticipantCreate) -> Result<()> {
        let url = &format!("{}/tournaments/{}/participants/{}.json",
                           API_BASE,
                           id.to_string(),
                           participant_id.0);
        let body = pairs_to_string(pc_to_pairs(participant));
        let _ = try!(retry(|| self.client.put(url)
                                         .headers(self.headers.clone())
                                         .body(&body)));
        Ok(())
    }

    /// Checks a participant in, setting checked_in_at to the current time.
    pub fn check_in_participant(&self,
                                id: &TournamentId,
                                participant_id: &ParticipantId) -> Result<()> {
        let url = &format!("{}/tournaments/{}/participants/{}/check_in.json",
                           API_BASE,
                           id.to_string(),
                           participant_id.0);
        let _ = try!(retry(|| self.client.post(url)
                                         .headers(self.headers.clone())));
        Ok(())
    }


    /// Marks a participant as having not checked in, setting checked_in_at to nil.
    pub fn undo_check_in_participant(&self,
                                     id: &TournamentId,
                                     participant_id: &ParticipantId) -> Result<()> {
        let url = &format!("{}/tournaments/{}/participants/{}/undo_check_in.json",
                           API_BASE,
                           id.to_string(),
                           participant_id.0);
        let _ = try!(retry(|| self.client.post(url)
                                         .headers(self.headers.clone())));
        Ok(())
    }

    /// If the tournament has not started, delete a participant, automatically filling in the abandoned seed number.
    /// If tournament is underway, mark a participant inactive, automatically forfeiting his/her remaining matches.
    pub fn delete_participant(&self,
                              id: &TournamentId,
                              participant_id: &ParticipantId) -> Result<()> {
        let url = &format!("{}/tournaments/{}/participants/{}.json",
                           API_BASE,
                           id.to_string(),
                           participant_id.0);
        let _ = try!(retry(|| self.client.delete(url)
                                         .headers(self.headers.clone())));
        Ok(())
    }

    /// Randomize seeds among participants. Only applicable before a tournament has started.
    pub fn randomize_participants(&self,
                                  id: &TournamentId) -> Result<()> {
        let url = &format!("{}/tournaments/{}/participants/randomize.json",
                           API_BASE,
                           id.to_string());
        let _ = try!(retry(|| self.client.post(url)
                                         .headers(self.headers.clone())));
        Ok(())
    }

    /// Retrieve a tournament's match list.
    pub fn match_index(&self,
                       id: &TournamentId,
                       state: Option<MatchState>,
                       participant_id: Option<ParticipantId>) -> Result<MatchIndex> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments/{}/matches.json",
                                                 API_BASE,
                                                 id.to_string())).unwrap();
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(s) = state {
                pairs.append_pair("state", &s.to_string());
            }
            if let Some(pid) = participant_id {
                pairs.append_pair("participant_id", &pid.0.to_string());
            }
        }
        let response = try!(retry(|| self.client.get(url.as_str())
                                                .headers(self.headers.clone())));
        MatchIndex::decode(try!(serde_json::from_reader(response)))
    }

    /// Retrieve a single match record for a tournament.
    pub fn get_match(&self,
                     id: &TournamentId,
                     match_id: &MatchId,
                     include_attachments: bool) -> Result<Match> {
        let mut url = url!("tournaments/{}/matches/{}.json",
                              API_BASE,
                              id.to_string(),
                              match_id.0)).unwrap();
        url.query_pairs_mut()
            .append_pair("include_attachments", &(include_attachments as i64).to_string());
        let response = try!(retry(|| self.client.get(url.as_str())
                                                .headers(self.headers.clone())));
        Match::decode(try!(serde_json::from_reader(response)))
    }

    /// Update/submit the score(s) for a match.
    pub fn update_match(&self,
                        id: &TournamentId,
                        match_id: &MatchId,
                        match_update: &MatchUpdate) -> Result<Match> {
        let url = &format!("{}/tournaments/{}/matches/{}.json",
                           API_BASE,
                           id.to_string(),
                           match_id.0);
        let body = pairs_to_string(mu_to_pairs(match_update));
        let response = try!(retry(|| self.client.put(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        Match::decode(try!(serde_json::from_reader(response)))
    }

    /// Retrieve a match's attachments.
    pub fn attachments_index(&self,
                             id: &TournamentId,
                             match_id: &MatchId) -> Result<AttachmentIndex> {
        let url = &format!("{}/tournaments/{}/matches/{}/attachments.json",
                           API_BASE,
                           id.to_string(),
                           match_id.0);
        let response = try!(retry(|| self.client.get(url)
                                                .headers(self.headers.clone())));
        AttachmentIndex::decode(try!(serde_json::from_reader(response)))
    }

    /// Retrieve a single match attachment record.
    pub fn get_attachment(&self,
                          id: &TournamentId,
                          match_id: &MatchId,
                          attachment_id: &AttachmentId) -> Result<Attachment> {
        let url = &format!("{}/tournaments/{}/matches/{}/attachments/{}.json",
                           API_BASE,
                           id.to_string(),
                           match_id.0,
                           attachment_id.0);
        let response = try!(retry(|| self.client.get(url)
                                                .headers(self.headers.clone())));
        Attachment::decode(try!(serde_json::from_reader(response)))
    }

    /// Add a file, link, or text attachment to a match. NOTE: The associated tournament's "accept_attachments" attribute must be true for this action to succeed.
    pub fn create_attachment(&self,
                             id: &TournamentId,
                             match_id: &MatchId,
                             attachment: &AttachmentCreate) -> Result<Attachment> {
        let url = &format!("{}/tournaments/{}/matches/{}/attachments.json",
                           API_BASE,
                           id.to_string(),
                           match_id.0);
        let body = pairs_to_string(at_to_pairs(attachment));
        let response = try!(retry(|| self.client.post(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        Attachment::decode(try!(serde_json::from_reader(response)))
    }

    /// Update the attributes of a match attachment.
    pub fn update_attachment(&self,
                             id: &TournamentId,
                             match_id: &MatchId,
                             attachment_id: &AttachmentId,
                             attachment: &AttachmentCreate) -> Result<Attachment> {
        let url = &format!("{}/tournaments/{}/matches/{}/attachments/{}.json",
                           API_BASE,
                           id.to_string(),
                           match_id.0,
                           attachment_id.0);
        let body = pairs_to_string(at_to_pairs(attachment));
        let response = try!(retry(|| self.client.put(url)
                                                .headers(self.headers.clone())
                                                .body(&body)));
        Attachment::decode(try!(serde_json::from_reader(response)))
    }

    /// Delete a match attachment.
    pub fn delete_attachment(&self,
                             id: &TournamentId,
                             match_id: &MatchId,
                             attachment_id: &AttachmentId) -> Result<()> {
        let url = &format!("{}/tournaments/{}/matches/{}/attachments/{}.json",
                           API_BASE,
                           id.to_string(),
                           match_id.0,
                           attachment_id.0);
        let _ = try!(retry(|| self.client.delete(url)
                                         .headers(self.headers.clone())));
        Ok(())
    }
    */

    // fn tournament_action(&self,
    //                      endpoint: &str,
    //                      id: &TournamentId,
    //                      includes: &TournamentIncludes) -> Result<()> {
        // let mut url = hyper::Url::parse(&format!("{}/tournaments/{}/{}.json",
        //                                          API_BASE,
        //                                          id.to_string(),
        //                                          endpoint)).unwrap();
        // Challonge::add_tournament_includes(&mut url, includes);
        // let _ = try!(retry(|| self.client.post(url.as_str()).headers(self.headers.clone())));
        // Ok(())
    // }

    // TODO refactor to be better
    // fn add_tournament_includes(url: &mut hyper::Url, includes: &TournamentIncludes) {
    //     let mut pairs = url.query_pairs_mut();
    //     match *includes {
    //         TournamentIncludes::All => {
    //             pairs
    //                 .append_pair("include_participants", "1")
    //                 .append_pair("include_matches", "1");
    //         },
    //         TournamentIncludes::Matches => {
    //             pairs
    //                 .append_pair("include_participants", "0")
    //                 .append_pair("include_matches", "1");
    //         },
    //         TournamentIncludes::Participants => {
    //             pairs
    //                 .append_pair("include_participants", "1")
    //                 .append_pair("include_matches", "0");
    //         },
    //     }
    // }
}

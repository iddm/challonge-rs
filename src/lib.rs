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
extern crate hyper;
extern crate serde_json;
extern crate chrono;

use chrono::date::Date;
use chrono::offset::local::Local;
pub mod tournament;
pub mod participants;
pub mod error;
pub mod matches;
use tournament::{
    Tournament,
    TournamentId,
    TournamentCreate,
    TournamentState,
    TournamentType,
    Index as TournamentIndex,
};
use participants::{
    Participant,
    Index as ParticipantIndex,
    ParticipantCreate,
    ParticipantId,
};
use matches::{
    Match,
    MatchState,
    MatchUpdate,
    Index as MatchIndex,
    MatchId,
};
use error::Error;

const API_BASE: &'static str = "https://api.challonge.com/v1";



fn check_status(response: hyper::Result<hyper::client::Response>)
    -> Result<hyper::client::Response, Error> {
    let response = try!(response);
    if !response.status.is_success() {
        return Err(Error::error_from_response(response))
    }
    Ok(response)
}

fn retry<'a, F: Fn() -> hyper::client::RequestBuilder<'a>>(f: F)
    -> Result<hyper::client::Response, Error> {
    let f2 = || check_status(f().send());
    // retry on a ConnectionAborted, which occurs if it's been a while since the last request
    match f2() {
        // Err(hyper::error::Error::Io(ref io))
        //     if io.kind() == std::io::ErrorKind::ConnectionAborted => f2(),
        other => other
    }
}

fn make_headers(user_name: String, api_key: String) -> hyper::header::Headers {
    let mut headers = hyper::header::Headers::new();
    headers.set(
       hyper::header::Authorization (
           hyper::header::Basic {
               username: user_name,
               password: Some(api_key),
           }
       )
    );
    headers
}

macro_rules! format_date {
    ($date:expr) => {
        $date.format("%Y-%m-%d").to_string()
    }
}

macro_rules! t {
    ($key:expr) => {
        concat!("tournament[", $key, "]")
    }
}

macro_rules! p {
    ($key:expr) => {
        concat!("participant[", $key, "]")
    }
}

macro_rules! m {
    ($key:expr) => {
        concat!("match[", $key, "]")
    }
}

fn pairs_to_string(params: Vec<(&'static str, String)>) -> String {
    let mut body = String::new();
    let mut sep = "";
    for p in params {
        body.push_str(sep);
        body.push_str(&format!("{}={}", p.0, p.1));
        sep = "&";
    }
    body
}

fn pc_to_pairs(participant: ParticipantCreate) -> Vec<(&'static str, String)>{
    let mut params = vec![
        (p!("email"), participant.email),
        (p!("seed"), participant.seed.to_string()),
        (p!("misc"), participant.misc),
    ];
   
    if let Some(n) = participant.name {
        params.push((p!("name"), n));
    }
    if let Some(un) = participant.challonge_username {
        params.push((p!("challonge_username"), un));
    }
    params
}
fn tc_to_pairs(tournament: &TournamentCreate) -> Vec<(&'static str, String)> {
    let mut params: Vec<(&'static str, String)> = vec![
        (t!("name"), tournament.name.clone()),
        (t!("tournament_type"), tournament.tournament_type.to_string()),
        (t!("url"), tournament.url.clone()),
        (t!("subdomain"), tournament.subdomain.clone()),
        (t!("description"), tournament.description.clone()),
        (t!("open_signup"), tournament.open_signup.to_string()),
        (t!("hold_third_place_match"), tournament.hold_third_place_match.to_string()),
        (t!("pts_for_match_win"), tournament.pts_for_match_win.to_string()),
        (t!("pts_for_match_tie"), tournament.pts_for_match_tie.to_string()),
        (t!("pts_for_game_win"), tournament.pts_for_game_win.to_string()),
        (t!("pts_for_game_tie"), tournament.pts_for_game_tie.to_string()),
        (t!("pts_for_bye"), tournament.pts_for_bye.to_string()),
        (t!("swiss_rounds"), tournament.swiss_rounds.to_string()),
        (t!("ranked_by"), tournament.ranked_by.to_string()),
        (t!("rr_pts_for_match_win"), tournament.rr_pts_for_match_win.to_string()),
        (t!("rr_pts_for_match_tie"), tournament.rr_pts_for_match_tie.to_string()),
        (t!("rr_pts_for_game_win"), tournament.rr_pts_for_game_win.to_string()),
        (t!("rr_pts_for_game_tie"), tournament.rr_pts_for_game_tie.to_string()),
        (t!("show_rounds"), tournament.show_rounds.to_string()),
        (t!("private"), tournament.private.to_string()),
        (t!("notify_users_when_matches_open"), tournament.notify_users_when_matches_open.to_string()),
        (t!("notify_users_when_the_tournament_ends"), tournament.notify_users_when_the_tournament_ends.to_string()),
        (t!("sequential_pairings"), tournament.sequential_pairings.to_string()),
        (t!("signup_cap"), tournament.signup_cap.to_string()),
        (t!("start_at"), tournament.start_at.to_rfc3339()),
        (t!("check_in_duration"), tournament.check_in_duration.to_string()),
    ];
    if let Some(gfm) = tournament.grand_finals_modifier.as_ref() {
        params.push((t!("grand_finals_modifier"), gfm.clone()));
    }
    params
}
fn mu_to_pairs(mu: MatchUpdate) -> Vec<(&'static str, String)>{
    let mut params = Vec::new();
    
    if let Some(v) = mu.player1_votes {
        params.push((m!("player1_votes"), v.to_string()));
    }
    if let Some(v) = mu.player2_votes {
        params.push((m!("player2_votes"), v.to_string()));
    }
    if !mu.scores_csv.is_empty() {
        params.push((m!("scores_csv"), mu.scores_csv));
    }
    if let Some(w) = mu.winner_id {
        params.push((m!("winner_id"), w.0.to_string()));
    }
    params
}

/// Client for the Challonge REST API.
pub struct Challonge {
    headers: hyper::header::Headers,
    client: hyper::client::Client,
}
impl Challonge {
    /// Create new connection to Challonge.
    /// # Example
    /// ```ignore
    /// extern crate challonge;
    ///
    /// use self::challonge::Challonge;
    ///
    /// let c = Challonge::new("myusername", "myapikey");
    /// ```
    pub fn new(user_name: &str, api_key: &str) -> Challonge {
        Challonge {
            client: hyper::Client::new(),
            headers: make_headers(user_name.to_owned(), api_key.to_owned()),
        }
    }

    /// Retrieve a set of tournaments created with your account. 
    /// # Example
    /// ```ignore
    /// extern crate challonge;
    /// extern crate chrono;
    ///
    /// use self::challonge::Challonge;
    /// use self::challonge::tournament::{ TournamentState, TournamentType };
    /// use self::chrono::*;
    ///
    /// let c = Challonge::new("myusername", "myapikey");
    /// let index = c.tournament_index (
    ///        &TournamentState::All,
    ///        &TournamentType::DoubleElimination,
    ///        &Local::today(),
    ///        &Local::today(),
    ///        "subdomain"
    /// );
    /// ```
    pub fn tournament_index(&self,
                            state: &TournamentState,
                            tournament_type: &TournamentType,
                            created_after: &Date<Local>,
                            created_before: &Date<Local>,
                            subdomain: &str) -> Result<TournamentIndex, Error> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments.json", API_BASE)).unwrap();
        url.query_pairs_mut()
            .append_pair("state", &state.to_string())
            .append_pair("type", &tournament_type.to_get_param())
            .append_pair("created_after", &format_date!(created_after))
            .append_pair("created_before", &format_date!(created_before))
            .append_pair("subdomain", subdomain);
        
        let response = try!(retry(|| self.client.get(url.as_str()).headers(self.headers.clone())));
        Ok(TournamentIndex::decode(try!(serde_json::from_reader(response))))
    }

    /// Retrieve a single tournament record created with your account. 
    /// # Example
    /// ```ignore
    /// extern crate challonge;
    ///
    /// use challonge::Challonge;
    ///
    /// let c = Challonge::new("myusername", "myapikey");
    /// let t = c.get_tournament(&TournamentId::Id(2669881), true, true);
    /// ```
    pub fn get_tournament(&self,
                          id: &TournamentId,
                          include_participants: bool,
                          include_matches: bool) -> Result<Tournament, Error> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments/{}.json", API_BASE, id.to_string())).unwrap();
        url.query_pairs_mut()
            .append_pair("include_participants", &(include_participants as i64).to_string())
            .append_pair("include_matches", &(include_matches as i64).to_string());
        
        let response = try!(retry(|| self.client.get(url.as_str())
                                        .headers(self.headers.clone())));
        Tournament::decode(try!(serde_json::from_reader(response)))
    }

    /// Create a new tournament. 
    /// # Example
    /// ```ignore
    /// extern crate challonge;
    ///
    /// use challonge::Challonge;
    /// use challonge::tournament::TournamentCreate;
    ///
    /// let c = Challonge::new("myusername", "myapikey");
    /// let tc = TournamentCreate {
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
    /// };
    /// let t = c.create_tournament(tc.clone());
    /// assert!(t.is_ok());
    /// ```
    pub fn create_tournament(&self, tournament: &TournamentCreate) -> Result<Tournament, Error> {
        let url = &format!("{}/tournaments.json", API_BASE);
        let body = pairs_to_string(tc_to_pairs(tournament));
        let response = try!(retry(|| self.client.post(url)
                                        .headers(self.headers.clone())
                                        .body(&body)));
        Tournament::decode(try!(serde_json::from_reader(response)))
    }
    
    /// Update a tournament's attributes. 
    pub fn update_tournament(&self,
                             id: TournamentId,
                             tournament: &TournamentCreate) -> Result<Tournament, Error> {
        let url = &format!("{}/tournaments/{}.json", API_BASE, id.to_string());
        let body = pairs_to_string(tc_to_pairs(tournament));
        let response = try!(retry(|| self.client.put(url)
                                        .headers(self.headers.clone())
                                        .body(&body)));
        Tournament::decode(try!(serde_json::from_reader(response)))
    }

    /// Deletes a tournament along with all its associated records. There is no undo, so use with care! 
    pub fn delete_tournament(&self, id: TournamentId) -> Result<(), Error> {
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
    /// NOTE: Checked in participants on the waiting list will be promoted if slots become available.
    pub fn tournament_process_checkins(&self,
                                       id: TournamentId,
                                       include_participants: bool,
                                       include_matches: bool) -> Result<(), Error> {
        self.tournament_action("process_check_ins", id, include_participants, include_matches)
    }

    /// When your tournament is in a 'checking_in' or 'checked_in' state, there's no way to edit the tournament's start time (start_at) or check-in duration (check_in_duration). You must first abort check-in, then you may edit those attributes.
    /// 
    /// 1. Makes all participants active and clears their checked_in_at times.
    /// 2. Transitions the tournament state from 'checking_in' or 'checked_in' to 'pending'
    pub fn tournament_abort_checkins(&self,
                                     id: TournamentId,
                                     include_participants: bool,
                                     include_matches: bool) -> Result<(), Error> {
        self.tournament_action("abort_check_in", id, include_participants, include_matches)
    }

    /// Start a tournament, opening up first round matches for score reporting. The tournament must have at least 2 participants. 
    pub fn tournament_start(&self,
                            id: TournamentId,
                            include_participants: bool,
                            include_matches: bool) -> Result<(), Error> {
        self.tournament_action("start", id, include_participants, include_matches)
    }
    
    /// Finalize a tournament that has had all match scores submitted, rendering its results permanent. 
    pub fn tournament_finalize(&self,
                               id: TournamentId,
                               include_participants: bool,
                               include_matches: bool) -> Result<(), Error> {
        self.tournament_action("finalize", id, include_participants, include_matches)
    }

    /// Reset a tournament, clearing all of its scores and attachments. You can then add/remove/edit participants before starting the tournament again. 
    pub fn tournament_reset(&self,
                            id: TournamentId,
                            include_participants: bool,
                            include_matches: bool) -> Result<(), Error> {
        self.tournament_action("reset", id, include_participants, include_matches)
    }

    /// Retrieve a tournament's participant list. 
    pub fn participant_index(&self,
                             id: TournamentId) -> Result<ParticipantIndex, Error> {
        let url = &format!("{}/tournaments/{}/participants.json",
                           API_BASE,
                           id.to_string());
        let response = try!(retry(|| self.client.get(url)
                                        .headers(self.headers.clone())));
        ParticipantIndex::decode(try!(serde_json::from_reader(response)))
    }
    
    /// Add a participant to a tournament (up until it is started). 
    pub fn create_participant(&self,
                              id: TournamentId,
                              participant: ParticipantCreate) -> Result<Participant, Error> {
        let url = &format!("{}/tournaments/{}/participants.json",
                           API_BASE,
                           id.to_string());
        let body = pairs_to_string(pc_to_pairs(participant));
        let response = try!(retry(|| self.client.post(url)
                                        .headers(self.headers.clone())
                                        .body(&body)));
        Participant::decode(try!(serde_json::from_reader(response)))
    }

    // / Bulk add participants to a tournament (up until it is started).
    // / If an invalid participant is detected, bulk participant creation will halt and any previously added participants (from this API request) will be rolled back. 
    // TODO
    // pub fn create_participant_bulk(&self,
    //                                id: TournamentId) -> Result<(), Error> {
    // }
    
    /// Retrieve a single participant record for a tournament.
    pub fn get_participant(&self,
                           id: TournamentId,
                           participant_id: ParticipantId,
                           include_matches: bool) -> Result<Participant, Error> {
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
                              id: TournamentId,
                              participant_id: ParticipantId,
                              participant: ParticipantCreate) -> Result<(), Error> {
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
                                id: TournamentId,
                                participant_id: ParticipantId) -> Result<(), Error> {
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
                                     id: TournamentId,
                                     participant_id: ParticipantId) -> Result<(), Error> {
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
                              id: TournamentId,
                              participant_id: ParticipantId) -> Result<(), Error> {
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
                                  id: TournamentId) -> Result<(), Error> {
        let url = &format!("{}/tournaments/{}/participants/randomize.json",
                           API_BASE,
                           id.to_string());
        let _ = try!(retry(|| self.client.post(url)
                                        .headers(self.headers.clone())));
        Ok(())
    }

    /// Retrieve a tournament's match list. 
    pub fn match_index(&self,
                       id: TournamentId,
                       state: Option<MatchState>,
                       participant_id: Option<ParticipantId>) -> Result<MatchIndex, Error> {
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
                     id: TournamentId,
                     match_id: MatchId,
                     include_attachments: bool) -> Result<Match, Error> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments/{}/matches/{}.json",
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
                        id: TournamentId,
                        match_id: MatchId,
                        match_update: MatchUpdate) -> Result<Match, Error> {
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

//     pub fn attachments_index(&self,
//                              id: TournamentId,
//                              match_id: MatchId) -> Result<Attachment, Error> {
//  https://api.challonge.com/v1/tournaments/{tournament}/matches/{match_id}/attachments.{json|xml}
//     }

    fn tournament_action(&self,
                         endpoint: &str,
                         id: TournamentId,
                         include_participants: bool,
                         include_matches: bool) -> Result<(), Error> {
        let mut url = hyper::Url::parse(&format!("{}/tournaments/{}/{}.json",
                                                 API_BASE,
                                                 id.to_string(),
                                                 endpoint)).unwrap();
        url.query_pairs_mut()
            .append_pair("include_participants", &(include_participants as i64).to_string())
            .append_pair("include_matches", &(include_matches as i64).to_string());
        let _ = try!(retry(|| self.client.post(url.as_str()).headers(self.headers.clone())));
        Ok(())
    }

    // fn prepare<'a>(&self, url: &str) -> hyper::client::RequestBuilder<'a> {
    //     self.client.get(url).headers(self.headers.clone())
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

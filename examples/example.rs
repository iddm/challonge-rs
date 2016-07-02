extern crate challonge;
extern crate chrono;

use challonge::Challonge;
use challonge::tournament::{
    RankedBy,
    TournamentId,
    TournamentCreate,
    TournamentType,
    TournamentState,
    TournamentIncludes,
};
use challonge::ParticipantCreate;
use chrono::*;

use std::ops::Add;

fn main() {
    let c = Challonge::new("myusername", "myapi_key");
    let i = c.tournament_index(
            &TournamentState::All,
            &TournamentType::DoubleElimination,
            &Local::today(),
            &Local::today(),
            "subdomain"
            );
    println!("Index: {:?}", i);

    let t = c.get_tournament(&TournamentId::Id(2669881), &TournamentIncludes::All);
    println!("Tournament: {:?}", t.unwrap());
    
    // let tc = TournamentCreate {
    //     name: "Tester".to_owned(),
    //     tournament_type: TournamentType::SingleElimination,
    //     url: "testerurl".to_owned(),
    //     subdomain: "subdomain".to_owned(),
    //     description: "Test tournament created from challonge-rs".to_owned(),
    //     open_signup: false,
    //     hold_third_place_match: false,
    //     pts_for_match_win: 0.0f64,
    //     pts_for_match_tie: 0.0f64,
    //     pts_for_game_win: 0.0f64,
    //     pts_for_game_tie: 0.0f64,
    //     pts_for_bye: 0.0f64,
    //     swiss_rounds: 0,
    //     ranked_by: RankedBy::PointsScored,
    //     rr_pts_for_match_win: 0.0f64,
    //     rr_pts_for_match_tie: 0.0f64,
    //     rr_pts_for_game_win: 0.0f64,
    //     rr_pts_for_game_tie: 0.0f64,
    //     show_rounds: false,
    //     private: false,
    //     notify_users_when_matches_open: true,
    //     notify_users_when_the_tournament_ends: true,
    //     sequential_pairings: false,
    //     signup_cap: 4,
    //     start_at: UTC::now().add(Duration::weeks(2)),
    //     check_in_duration: 60,
    //     grand_finals_modifier: None,
    // };
    let mut tc = TournamentCreate::new();
        tc.name("Test tournament")
          .tournament_type(TournamentType::SingleElimination)
          .url("TestUrl")
          .subdomain("subdomain")
          .description("TEST TOURNAMENT created by challonge-rs");
    let t = c.create_tournament(&tc);
    println!("Created tournament: {:?}", t);


    let tt = c.update_tournament(&TournamentId::Id(2674470), &tc);
    println!("Updated tournament: {:?}", tt);

    println!("Delete result: {:?}", c.delete_tournament(&TournamentId::Id(2674588)));

    println!("Check-in process result: {:?}", c.tournament_process_checkins(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()), &TournamentIncludes::All));
    println!("Check-in abort result: {:?}", c.tournament_abort_checkins(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()), &TournamentIncludes::All));
    println!("Start result: {:?}", c.tournament_start(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()), &TournamentIncludes::All));
    println!("Finalize result: {:?}", c.tournament_finalize(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()), &TournamentIncludes::All));
    println!("Reset result: {:?}", c.tournament_reset(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()), &TournamentIncludes::All));

    let pc = ParticipantCreate {
        // name: "username".to_owned(),
        name: None,
        challonge_username: None,
        email: "mail@themail.com".to_owned(),
        seed: 1,
        misc: "PIZDEC ON KRASAVCHIK".to_owned(),
    };
    println!("Participant result: {:?}", c.create_participant(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()),
    &pc));

    println!("Matches: {:?}", c.match_index(&TournamentId::Url("subdomain".to_owned(), "test1".to_owned()), None, None));
}

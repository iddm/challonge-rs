use chrono::*;

use ::*;


#[derive(Debug, Clone, Serialize)]
pub struct TournamentIndexFilter {
    state: TournamentState,
    tournament_type: TournamentType,
    created_after: NaiveDate,
    created_before: NaiveDate,
    subdomain: String,
}
// impl Default for TournamentIndexFilter {
//     fn default() -> TournamentIndexFilter {
//         TournamentIndexFilter {
//            state: TournamentState::
//         }
//     }
// }

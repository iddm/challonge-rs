use chrono::*;

use ::*;


#[derive(Debug, Clone, Serialize)]
pub struct TournamentIndexFilter {
    state: TournamentState,
    tournament_type: TournamentType,
    created_after: Date<Utc>,
    created_before: Date<Utc>,
    subdomain: String,
}
// impl Default for TournamentIndexFilter {
//     fn default() -> TournamentIndexFilter {
//         TournamentIndexFilter {
//            state: TournamentState::
//         }
//     }
// }

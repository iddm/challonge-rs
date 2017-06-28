use std::fmt;

use ::*;


const API_BASE: &'static str = "https://api.challonge.com/v1";


#[derive(Debug, Clone)]
pub enum Endpoint {
    TournamentIndex,
    GetTournament {
        id: TournamentId,
        includes: TournamentIncludes,
    },
    // GetMatch {
    //     id: TournamentId,
    //     match_id: MatchId,
    //     include_attachments: bool,
    // },
}

impl fmt::Display for Endpoint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let address = match *self {
            Endpoint::TournamentIndex => format!("/tournaments.json"),
            Endpoint::GetTournament { ref id, includes } => format!("/tournaments/{}.json",
                                                                    id.to_string()),
            // Endpoint::GetMatch { ref id, ref match_id, ref include_attachments } => 
        };
        fmt.write_str(&format!("{}{}", API_BASE, address))
    }
}

//! Challonge Participant type.

extern crate serde_json;

use chrono::*;
use serde_json::Value;

use error::Error;
use util::{decode_array, into_map, remove};

/// Represents an ID of a participant
#[derive(Debug, Clone, PartialEq)]
pub struct ParticipantId(pub u64);

/// A structure for creating a participant (adding the participant to the tournament).
#[derive(Debug, Clone)]
pub struct ParticipantCreate {
    /// The name displayed in the bracket/schedule - not required if email or challonge_username is provided. Must be unique per tournament.
    pub name: Option<String>,

    /// Provide this if the participant has a Challonge account. He or she will be invited to the tournament.
    pub challonge_username: Option<String>,

    /// Providing this will first search for a matching Challonge account.
    /// If one is found, this will have the same effect as the "challonge_username" attribute.
    /// If one is not found, the "new-user-email" attribute will be set, and the user will be invited via email to create an account.
    pub email: String,

    /// The participant's new seed.
    /// Must be between 1 and the current number of participants (including the new record).
    /// Overwriting an existing seed will automatically bump other participants as you would expect.
    pub seed: u64,

    /// Max: 255 characters. Multi-purpose field that is only visible via the API and handy for site integration (e.g. key to your users table).
    pub misc: String,
}
impl ParticipantCreate {
    /// Creates a structure to create participant with default values.
    pub fn new() -> ParticipantCreate {
        ParticipantCreate {
            name: None,
            challonge_username: None,
            email: String::default(),
            seed: 1,
            misc: String::default(),
        }
    }

    builder_so!(name);
    builder_so!(challonge_username);
    builder_s!(email);
    builder!(seed, u64);
    builder_s!(misc);
}

impl Default for ParticipantCreate {
    fn default() -> Self {
        Self::new()
    }
}

/// A list of participants for the tournament.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Index(pub Vec<Participant>);
impl Index {
    /// Decodes participants index from JSON.
    pub fn decode(value: Value) -> Result<Index, Error> {
        Ok(Index(decode_array(value, Participant::decode)?))
    }
}

/// Challonge `Participant` definition.
#[derive(Debug, Clone)]
pub struct Participant {
    /// Is a participant active
    pub active: bool,

    /// Time when the participant was checked in
    pub checked_in_at: Option<DateTime<FixedOffset>>,

    /// Time when the participant was added to the tournament
    pub created_at: DateTime<FixedOffset>,

    /// ???
    pub final_rank: Option<u64>,

    /// ???
    pub group_id: Option<u64>,

    /// ???
    pub icon: String,

    /// Unique participant identifier
    pub id: ParticipantId,

    /// Invitation id.
    pub invitation_id: Option<u64>,

    /// Invitation email.
    pub invite_email: String,

    /// ???
    pub misc: String,

    /// Name of the participant.
    pub name: String,

    /// ???
    pub on_waiting_list: bool,

    /// Seed of the participant in the tournament.
    pub seed: u64,

    /// Id of the tournament the participant belongs to.
    pub tournament_id: u64,

    /// Time when the participant was updated last time
    pub updated_at: DateTime<FixedOffset>,

    /// A name of a user in challonge system.
    pub challonge_username: String,

    /// Verified email address in challonge system.
    pub challonge_email_address_verified: String,

    /// Is the participant can be removed
    pub removable: bool,

    /// ???
    pub participatable_or_invitation_attached: bool,

    /// Needs removal confirmation
    pub confirm_remove: bool,

    /// Participant has invitation pending yet.
    pub invitation_pending: bool,

    /// ???
    pub display_name_with_invitation_email_address: String,

    /// ???
    pub email_hash: String,

    /// ???
    pub username: String,

    /// ???
    pub attached_participatable_portrait_url: String,

    /// Is the participant able to check in
    pub can_check_in: bool,

    /// Did the participant check in
    pub checked_in: bool,

    /// Participant can be reactivated
    pub reactivatable: bool,
}
impl Participant {
    /// Decodes `Participant` from JSON.
    pub fn decode(value: Value) -> Result<Participant, Error> {
        let mut value = into_map(value)?;
        let t = remove(&mut value, "participant")?;
        let mut tv = into_map(t)?;

        let mut checked_in_at = None;
        if let Some(ci_str) = remove(&mut tv, "checked_in_at")?.as_string() {
            if let Ok(ci) = DateTime::parse_from_rfc3339(ci_str) {
                checked_in_at = Some(ci);
            }
        }

        Ok(Participant {
            active: remove(&mut tv, "active")?.as_boolean().unwrap_or(false),
            checked_in_at,
            created_at: DateTime::parse_from_rfc3339(
                remove(&mut tv, "created_at")?.as_string().unwrap_or(""),
            )
            .unwrap(),
            final_rank: remove(&mut tv, "final_rank")?.as_u64(),
            group_id: remove(&mut tv, "group_id")?.as_u64(),
            icon: remove(&mut tv, "icon")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            id: ParticipantId(remove(&mut tv, "id")?.as_u64().unwrap()),
            invitation_id: remove(&mut tv, "invitation_id")?.as_u64(),
            invite_email: remove(&mut tv, "invite_email")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            misc: remove(&mut tv, "misc")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            name: remove(&mut tv, "name")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            on_waiting_list: remove(&mut tv, "on_waiting_list")?
                .as_boolean()
                .unwrap_or(false),
            seed: remove(&mut tv, "seed")?.as_u64().unwrap(),
            tournament_id: remove(&mut tv, "tournament_id")?.as_u64().unwrap(),
            updated_at: DateTime::parse_from_rfc3339(
                remove(&mut tv, "updated_at")?.as_string().unwrap_or(""),
            )
            .unwrap(),
            challonge_username: remove(&mut tv, "challonge_username")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            challonge_email_address_verified: remove(&mut tv, "challonge_email_address_verified")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            removable: remove(&mut tv, "removable")?.as_boolean().unwrap_or(false),
            participatable_or_invitation_attached: remove(
                &mut tv,
                "participatable_or_invitation_attached",
            )?
            .as_boolean()
            .unwrap_or(false),
            confirm_remove: remove(&mut tv, "confirm_remove")?
                .as_boolean()
                .unwrap_or(false),
            invitation_pending: remove(&mut tv, "invitation_pending")?
                .as_boolean()
                .unwrap_or(false),
            display_name_with_invitation_email_address: remove(
                &mut tv,
                "display_name_with_invitation_email_address",
            )?
            .as_string()
            .unwrap_or("")
            .to_owned(),
            email_hash: remove(&mut tv, "email_hash")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            username: remove(&mut tv, "username")?
                .as_string()
                .unwrap_or("")
                .to_owned(),
            attached_participatable_portrait_url: remove(
                &mut tv,
                "attached_participatable_portrait_url",
            )?
            .as_string()
            .unwrap_or("")
            .to_owned(),
            checked_in: remove(&mut tv, "checked_in")?.as_boolean().unwrap_or(false),
            can_check_in: remove(&mut tv, "can_check_in")?
                .as_boolean()
                .unwrap_or(false),
            reactivatable: remove(&mut tv, "reactivatable")?
                .as_boolean()
                .unwrap_or(false),
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use participants::Participant;

    #[test]
    fn test_participant_parse() {
        let string = r#"{
          "participant": {
            "active": true,
            "checked_in_at": null,
            "created_at": "2015-01-19T16:54:40-05:00",
            "final_rank": null,
            "group_id": null,
            "icon": null,
            "id": 16543993,
            "invitation_id": null,
            "invite_email": null,
            "misc": null,
            "name": "Participant #1",
            "on_waiting_list": false,
            "seed": 1,
            "tournament_id": 1086875,
            "updated_at": "2015-01-19T16:54:40-05:00",
            "challonge_username": null,
            "challonge_email_address_verified": null,
            "removable": true,
            "participatable_or_invitation_attached": false,
            "confirm_remove": true,
            "invitation_pending": false,
            "display_name_with_invitation_email_address": "Participant #1",
            "email_hash": null,
            "username": null,
            "attached_participatable_portrait_url": null,
            "can_check_in": false,
            "checked_in": false,
            "reactivatable": false
          }
        }"#;
        let json_r = serde_json::from_str(string);
        assert!(json_r.is_ok());
        let json = json_r.unwrap();
        if let Ok(p) = Participant::decode(json) {
            assert_eq!(p.active, true);
            assert_eq!(p.checked_in_at, None);
            // assert_eq!(p.created_at, );
            assert_eq!(p.final_rank, None);
            assert_eq!(p.group_id, None);
            assert!(p.icon.is_empty());
            assert_eq!(p.id.0, 16543993);
            assert_eq!(p.invitation_id, None);
            assert!(p.invite_email.is_empty());
            assert!(p.misc.is_empty());
            assert_eq!(p.name, "Participant #1");
            assert_eq!(p.on_waiting_list, false);
            assert_eq!(p.seed, 1);
            assert_eq!(p.tournament_id, 1086875);
            // assert_eq!(p.updated_at, );
            assert!(p.challonge_username.is_empty());
            assert!(p.challonge_email_address_verified.is_empty());
            assert_eq!(p.removable, true);
            assert_eq!(p.participatable_or_invitation_attached, false);
            assert_eq!(p.confirm_remove, true);
            assert_eq!(p.invitation_pending, false);
            assert_eq!(
                p.display_name_with_invitation_email_address,
                "Participant #1"
            );
            assert!(p.email_hash.is_empty());
            assert!(p.username.is_empty());
            assert!(p.attached_participatable_portrait_url.is_empty());
            assert_eq!(p.can_check_in, false);
            assert_eq!(p.checked_in, false);
            assert_eq!(p.reactivatable, false);
        } else {
            assert!(false);
        }
    }
}

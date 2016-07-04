//! Challonge Attachment type.

extern crate serde_json;

use serde_json::Value;
use chrono::*;
use std::collections::BTreeMap;

use ::decode_array;
use matches::MatchId;
use error::Error;

fn into_map(value: Value) -> Result<BTreeMap<String, serde_json::Value>, Error> {
    match value {
        Value::Object(m) => Ok(m),
        value => Err(Error::Decode("Expected object", value)),
    }
}

fn remove(map: &mut BTreeMap<String, Value>, key: &str) -> Result<Value, Error> {
    map.remove(key).ok_or(Error::Decode("Unexpected absent key", Value::String(key.into())))
}

/// Asset of a attachment
#[derive(Debug, Clone)]
pub struct Asset {
    /// File name of an attachment.
    pub file_name: Option<String>,

    /// Content type (MIME-type).
    pub content_type: Option<String>,

    /// Size of a file attached.
    pub file_size: Option<u64>,

    /// ???
    pub url: Option<String>,
}
impl Asset {
    /// Decodes `Asset` from `Attachment`'s JSON
    pub fn decode(mut map: &mut BTreeMap<String, Value>) -> Result<Asset, Error> {
        Ok(Asset {
            file_name: try!(remove(&mut map, "asset_file_name")).as_string().map_or(None, |f| Some(f.to_owned())),
            content_type: try!(remove(&mut map, "asset_content_type")).as_string().map_or(None, |f| Some(f.to_owned())),
            file_size: try!(remove(&mut map, "asset_file_size")).as_u64(),
            url: try!(remove(&mut map, "asset_url")).as_string().map_or(None, |f| Some(f.to_owned())),
        })
    }
}

/// A structure for creating an attachment
/// * At least 1 of the 3 optional parameters must be provided.
/// * Files up to 25MB are allowed for tournaments hosted by Premier badge Challonge Premier subscribers.
pub struct AttachmentCreate {

    /// A file upload (250KB max, no more than 4 attachments per match). If provided, the url parameter will be ignored. 
    pub asset: Option<Vec<u8>>,

    /// A web (http, ftp) link
    pub url: Option<String>,

    /// Text to describe the file or URL attachment, or this can simply be standalone text. 
    pub description: Option<String>,
}
impl AttachmentCreate {
    /// Creates new `AttachmentCreate` structure with default values.
    pub fn new() -> AttachmentCreate {
        AttachmentCreate {
            asset: None,
            url: None,
            description: None,
        }
    }

    builder_o!(asset, Vec<u8>);
    builder_so!(url);
    builder_so!(description);
}

/// Unique attachment id 
#[derive(Debug, Clone)]
pub struct AttachmentId(pub u64);

/// Challonge `Attachment` definition.
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Unique attachment identifier 
    pub id: AttachmentId,

    /// Unique match identifier 
    pub match_id: MatchId,

    /// ??? 
    pub user_id: u64,

    /// A web (http, ftp) link
    pub url: Option<String>,

    /// Description of an attachment 
    /// Text to describe the file or URL attachment, or this can simply be standalone text. 
    pub description: Option<String>,

    /// Original attachment file name.
    pub original_file_name: Option<String>,

    /// Time when the attachment was created.
    pub created_at: DateTime<FixedOffset>,

    /// Time when the attachment was updated last time. 
    pub updated_at: DateTime<FixedOffset>,

    /// Asset information
    pub asset: Asset,
}
impl Attachment {
    /// Decodes `Attachment` from JSON
    pub fn decode(value: Value) -> Result<Attachment, Error> {
        let mut value = try!(into_map(value));
        let t = try!(remove(&mut value, "match_attachment"));
        let mut tv = try!(into_map(t));

        Ok(Attachment {
            id: AttachmentId(try!(remove(&mut tv, "id")).as_u64().unwrap()),
            match_id: MatchId(try!(remove(&mut tv, "match_id")).as_u64().unwrap()),
            user_id: try!(remove(&mut tv, "user_id")).as_u64().unwrap(),
            description: try!(remove(&mut tv, "description")).as_string().map_or(None, |f| Some(f.to_owned())),
            url: try!(remove(&mut tv, "url")).as_string().map_or(None, |f| Some(f.to_owned())),
            original_file_name: try!(remove(&mut tv, "original_file_name")).as_string().map_or(None, |f| Some(f.to_owned())),
            created_at: DateTime::parse_from_rfc3339(try!(remove(&mut tv, "created_at")).as_string().unwrap_or("")).unwrap(),
            updated_at: DateTime::parse_from_rfc3339(try!(remove(&mut tv, "updated_at")).as_string().unwrap_or("")).unwrap(),
            asset: Asset::decode(&mut tv).unwrap(),
        })
    }
}

/// Challonge Attachment index definition.
#[derive(Debug, Clone)]
pub struct Index(pub Vec<Attachment>);

impl Index {
    /// Decodes attachment index from JSON.
    pub fn decode(value: Value) -> Result<Index, Error> {
        Ok(Index(try!(decode_array(value, Attachment::decode))))
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use attachments::{
        Attachment,
        Index,
    };


    #[test]
    fn test_attachment_parse() {
        let string = r#"{"match_attachment":{"id":165418,"match_id":65187924,"user_id":979950,"description":"discord","url":"","original_file_name":null,"created_at":"2016-07-02T13:24:09.899-04:00","updated_at":"2016-07-02T13:24:09.899-04:00","asset_file_name":null,"asset_content_type":null,"asset_file_size":null,"asset_url":null}}"#;
        let json_r = serde_json::from_str(string);
        assert!(json_r.is_ok());
        let json = json_r.unwrap();
        if let Ok(m) = Attachment::decode(json) {
            assert_eq!(m.id.0, 165418);
            assert_eq!(m.match_id.0, 65187924);
            assert_eq!(m.user_id, 979950);
            assert_eq!(m.description, Some("discord".to_owned()));
            assert_eq!(m.url, Some("".to_owned()));
            assert_eq!(m.original_file_name, None); 
            assert_eq!(m.asset.file_name, None); 
            assert_eq!(m.asset.content_type, None); 
            assert_eq!(m.asset.file_size, None); 
            assert_eq!(m.asset.url, None); 
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_index_parse() {
        let string = r#"[{"match_attachment":{"id":165418,"match_id":65187924,"user_id":979950,"description":"discord","url":"","original_file_name":null,"created_at":"2016-07-02T13:24:09.899-04:00","updated_at":"2016-07-02T13:24:09.899-04:00","asset_file_name":null,"asset_content_type":null,"asset_file_size":null,"asset_url":null}},{"match_attachment":{"id":165417,"match_id":65187924,"user_id":979950,"description":"test description","url":"","original_file_name":null,"created_at":"2016-07-02T13:21:14.794-04:00","updated_at":"2016-07-02T13:21:14.794-04:00","asset_file_name":null,"asset_content_type":null,"asset_file_size":null,"asset_url":null}}]"#;
        let json_r = serde_json::from_str(string);
        assert!(json_r.is_ok());
        let json = json_r.unwrap();
        if let Ok(i) = Index::decode(json) {
            assert_eq!(i.0.len(), 2);
        } else {
            assert!(false);
        }
    }
}

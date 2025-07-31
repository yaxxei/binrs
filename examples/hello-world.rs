use std::collections::{BTreeSet, HashMap};

use binrs::{
    decoder::{Decode},
    encoder::{Encode},
    error::Error,
};
use binrs_derive::{Decode, Encode};

fn main() {
    let user = User {
        id: 1001,
        username: "johndoe".to_string(),
        email: Some("john@example.com".to_string()),
        age: Some(30),
        is_active: true,
        roles: vec!["admin".to_string(), "user".to_string()],
        settings: {
            let mut map = HashMap::new();
            map.insert("theme".to_string(), "dark".to_string());
            map.insert("lang".to_string(), "en".to_string());
            map
        },
        notifications_enabled: Ok(true),
        tags: ["rustacean", "verified"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        last_login: Some(1_722_145_600),
        ignored: "Ignored".to_string(),
    };

    let bytes = user.encode_to_bytes().unwrap();
    println!("{bytes:x?}");

    let user = User::decode_from_bytes(&bytes).unwrap();
    println!("{user:#?}");
}

#[derive(Debug, Encode, Decode)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
    pub age: Option<u8>,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub settings: HashMap<String, String>,
    pub notifications_enabled: Result<bool, String>,
    pub tags: BTreeSet<String>,
    pub last_login: Option<u64>,
    #[bin(skip)]
    pub ignored: String,
}


use std::collections::{BTreeSet, HashMap};

use binrs::{
    decoder::{Decode, Decoder},
    encoder::{Encode, Encoder},
    error::Error,
};

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
        last_login: Some(1_722_145_600), // пример Unix-времени
    };

    let bytes = user.encode_to_bytes().unwrap();
    println!("{bytes:x?}");

    let user = User::decode_from_bytes(&bytes).unwrap();
    println!("{user:#?}");
}

#[derive(Debug, PartialEq)]
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
}

impl Encode for User {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.id.encode(encoder)?;
        self.username.encode(encoder)?;
        self.email.encode(encoder)?;
        self.age.encode(encoder)?;
        self.is_active.encode(encoder)?;
        self.roles.encode(encoder)?;
        self.settings.encode(encoder)?;
        self.notifications_enabled.encode(encoder)?;
        self.tags.encode(encoder)?;
        self.last_login.encode(encoder)
    }
}

impl Decode for User {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Self {
            id: u64::decode(decoder)?,
            username: String::decode(decoder)?,
            email: Option::<String>::decode(decoder)?,
            age: Option::<u8>::decode(decoder)?,
            is_active: bool::decode(decoder)?,
            roles: Vec::<String>::decode(decoder)?,
            settings: HashMap::<String, String>::decode(decoder)?,
            notifications_enabled: Result::<bool, String>::decode(decoder)?,
            tags: BTreeSet::<String>::decode(decoder)?,
            last_login: Option::<u64>::decode(decoder)?,
        })
    }
}

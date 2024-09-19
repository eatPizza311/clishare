use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        // benifit from derive_more From implementation
        Uuid::new_v4().into()
    }

    // Forwarding Uuid's nil function, which will create all zeros uuid
    // to represent a non-exist or invalid uuid
    // This also use to block out database id when it's serialized and sent across network
    // because no client should know about database id (It's totally internal!)
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}

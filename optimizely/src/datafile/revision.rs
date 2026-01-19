// External imports
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::ops::Deref;

#[derive(Debug)]
pub struct Revision(u32);

impl<'de> Deserialize<'de> for Revision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // A revision is stored as a String type, ...
        let revision = String::deserialize(deserializer)?;

        // ... but should be parsed as an unsigned integer
        let revision = match revision.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::custom("expected revision to be parseable as int"));
            }
        };

        Ok(Revision(revision))
    }
}

impl Deref for Revision {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

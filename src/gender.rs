//! Gender type

use crate::Error;
use serde::{Deserialize, Serialize, de::Deserializer};
use std::fmt;
use std::str::FromStr;

/// A persons gender
#[derive(Debug, PartialEq, Serialize)]
pub enum Gender{
    /// Male gender
    Male,
    /// Female gender
    Female,
    /// Other type of gender
    Other
}

impl Gender {
    /// Deserialize
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Gender, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        match buf.as_str() {
            "Male" | "male" => Ok(Gender::Male),
	    "Female" | "female" => Ok(Gender::Female),
	    "Other" | "other" => Ok(Gender::Other),
            _ => Err(Error::Gender).map_err(serde::de::Error::custom),
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let media_type = match self {
            Gender::Male => "Male",
	    Gender::Female => "Female",
	    Gender::Other => "Other",
        };
        writeln!(f, "{media_type}")
    }
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Male" | "male" => Ok(Gender::Male),
	    "Female" | "female" => Ok(Gender::Female),
	    "Other" | "other" => Ok(Gender::Other),
	    _ => Err(Error::Gender),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq, Serialize)]
    struct Med {
        #[serde(deserialize_with = "Gender::deserialize")]
        pub media_type: Gender,
    }

    #[test]
    fn test_desert_media_type() {
        let mt = Med {
            media_type: Gender::Male,
        };
        let j = serde_json::to_string(&mt).unwrap();
        let mt2: Med = serde_json::from_str(&j).unwrap();

        assert_eq!(mt, mt2);
    }
}

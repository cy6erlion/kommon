//! Gender type

use crate::Error;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{de::Deserializer, ser::Serializer, Deserialize};

#[cfg(feature = "rusqlite")]
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

/// A persons gender
#[derive(PartialEq, Debug, Clone)]
pub enum Gender {
    /// Male gender
    Male,
    /// Female gender
    Female,
    /// Other type of gender
    Other,
}

impl Gender {
    /// Deserialize
    #[cfg(feature = "serde")]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Gender, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;
        let gender: Gender = buf.trim().parse().map_err(serde::de::Error::custom)?;
        Ok(gender)
    }

    /// Serialize
    #[cfg(feature = "serde")]
    pub fn serialize<S>(gender: &Gender, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&gender.to_string())
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Male" => Ok(Gender::Male),
            "Female" => Ok(Gender::Female),
            "Other" => Ok(Gender::Other),
            _ => Err(Error::Gender),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl ToSql for Gender {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}
#[cfg(feature = "rusqlite")]
impl FromSql for Gender {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "serde")]
    #[derive(Deserialize, Debug, PartialEq, Serialize)]
    struct Med {
        #[serde(deserialize_with = "Gender::deserialize")]
        #[serde(serialize_with = "Gender::serialize")]
        gender: Gender,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_gender() {
        let mt = Med {
            gender: Gender::Male,
        };
        let j = serde_json::to_string(&mt).unwrap();
        let mt2: Med = serde_json::from_str(&j).unwrap();
        assert_eq!(mt.gender.to_string(), mt2.gender.to_string());
    }
}

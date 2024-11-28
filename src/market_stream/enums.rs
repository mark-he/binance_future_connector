use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};
use strum::Display;

#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize)]
pub enum UpdateSpeed {
    #[strum(serialize = "100ms")]
    Faster,
    #[strum(serialize = "500ms")]
    Slower,
}
impl FromStr for UpdateSpeed {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "100ms" => Ok(UpdateSpeed::Faster),
            "500ms" => Ok(UpdateSpeed::Slower),
            _ => Err("Invalid UpdateSpeed".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for UpdateSpeed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}
#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize)]
pub enum Level {
    #[strum(serialize = "5")]
    L5,
    #[strum(serialize = "10")]
    L10,
    #[strum(serialize = "20")]
    L20,
}
impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "5" => Ok(Level::L5),
            "10" => Ok(Level::L10),
            "20" => Ok(Level::L20),
            _ => Err("Invalid Level".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}
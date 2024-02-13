use crate::user::SubtitleMode;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(mode: &SubtitleMode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mode_str = match mode {
        SubtitleMode::Default => "Default",
        SubtitleMode::Always => "Always",
        SubtitleMode::OnlyForced => "OnlyForced",
        SubtitleMode::None => "None",
        SubtitleMode::Smart => "Smart",
    };
    serializer.serialize_str(mode_str)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<SubtitleMode, D::Error>
where
    D: Deserializer<'de>,
{
    let mode_str = String::deserialize(deserializer)?;
    match mode_str.as_ref() {
        "Default" => Ok(SubtitleMode::Default),
        "Always" => Ok(SubtitleMode::Always),
        "OnlyForced" => Ok(SubtitleMode::OnlyForced),
        "None" => Ok(SubtitleMode::None),
        "Smart" => Ok(SubtitleMode::Smart),
        _ => Err(serde::de::Error::custom("Invalid subtitle mode")),
    }
}

use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Weather conditions in the session
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq)]
#[repr(u8)]
pub enum Weather
{
    /// Clear weather
    #[default]
    Clear = 0,
    /// Light cloud cover
    LightCloud = 1,
    /// Overcast conditions
    Overcast = 2,
    /// Light rain
    LightRain = 3,
    /// Heavy rain
    HeavyRain = 4,
    /// Thunderstorm
    Storm = 5,
    /// Unknown weather condition
    Unknown = 6,
}

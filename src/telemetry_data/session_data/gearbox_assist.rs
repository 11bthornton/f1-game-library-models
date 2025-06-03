use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Gearbox assist settings
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum GearboxAssist
{
    /// Manual gearbox (no assist)
    Manual,
    /// Manual gearbox with suggested gear indicators
    #[serde(rename = "Suggested Gear")]
    SuggestedGear,
    /// Fully automatic gearbox
    Automatic,
}

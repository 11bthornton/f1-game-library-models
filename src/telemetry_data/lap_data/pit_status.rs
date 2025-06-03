use serde::Serialize;
use serde_repr::Deserialize_repr;

/// Pit status.
///
/// These represent the various pit-related states a car can be in.
#[derive(Deserialize_repr, Debug, Default, Serialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PitStatus {
    /// Not in pit area
    #[default]
    #[serde(rename = "None")]
    None = 0,

    /// Currently entering or in pit lane
    #[serde(rename = "Pitting")]
    Pitting = 1,

    /// In pit box area
    #[serde(rename = "In Pit Area")]
    InPitArea = 2,
}

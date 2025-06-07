use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Visual tyre compound types.
///
/// These are the tyre compounds as they appear visually in the game,
/// which can be different from the actual compound used.
#[derive(Deserialize_repr, Debug, Serialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum VisualTyreCompound {
    /// No tyre compound
    None = 0,

    /// Soft compound (red)
    #[serde(rename = "SOFT")]
    Soft = 16,

    /// Medium compound (yellow)
    #[serde(rename = "MED")]
    Medium = 17,

    /// Hard compound (white)
    #[serde(rename = "HARD")]
    Hard = 18,

    /// Intermediate compound (green)
    #[serde(rename = "INTER")]
    Inter = 7,

    /// Wet compound (blue)
    #[serde(rename = "WET")]
    Wet = 8,

    /// Classic dry compound
    #[serde(rename = "DRY")]
    ClassicDry = 9,

    /// Classic wet compound
    #[serde(rename = "WET")]
    ClassicWet = 10,

    /// F2 2019 wet compound
    #[serde(rename = "WET")]
    F220Wet = 15,

    /// F2 2019 super soft compound
    #[serde(rename = "SS")]
    F220SuperSoft = 19,

    /// F2 soft compound
    #[serde(rename = "SOFT")]
    F2Soft = 20,

    /// F2 medium compound
    #[serde(rename = "MED")]
    F2Medium = 21,

    /// F2 hard compound
    #[serde(rename = "HARD")]
    F2Hard = 22,
}

/// Actual tyre compound types.
///
/// These are the actual tyre compounds used by the car,
/// which can be different from how they appear visually.
#[derive(Deserialize_repr, Debug, Serialize, Clone, Copy)]
#[repr(u8)]
pub enum ActualTyreCompound {
    /// No tyre compound (for non-player cars)
    None = 0,
    /// C5 compound (softest)
    C5 = 16,
    /// C4 compound
    C4 = 17,
    /// C3 compound
    C3 = 18,
    /// C2 compound
    C2 = 19,
    /// C1 compound (hardest)
    C1 = 20,
    /// C0 compound (development/special)
    C0 = 21,
    /// C6 compound
    C6 = 22,
    /// Intermediate compound
    Inter = 7,
    /// Wet compound
    Wet = 8,
    /// Classic dry compound
    ClassicDry = 9,
    /// Classic wet compound
    ClassicWet = 10,
    /// F2 super soft compound
    F2SuperSoft = 11,
    /// F2 soft compound
    F2Soft = 12,
    /// F2 medium compound
    F2Medium = 13,
    /// F2 hard compound
    F2Hard = 14,
    /// F2 2015 compound
    F215 = 15,
}

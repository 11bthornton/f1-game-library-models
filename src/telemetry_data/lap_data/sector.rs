//! Defines the `Sector` enum which represents the sectors of a track.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Track sectors.
///
/// These represent the three timing sectors of a track.
#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Sector {
    /// First sector of the track
    #[default]
    #[serde(rename = "Sector One")]
    SectorOne = 0,

    /// Second sector of the track
    #[serde(rename = "Sector Two")]
    SectorTwo = 1,

    /// Third sector of the track
    #[serde(rename = "Sector Three")]
    SectorThree = 2,
}

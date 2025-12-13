//! Defines the experience mode for various session features.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Experience mode setting for safety car, formation lap, etc.
///
/// Controls how certain race events are presented to the player.
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExperienceMode {
    /// Broadcast-style presentation with TV-like camera angles
    #[default]
    Broadcast = 0,
    /// Immersive presentation from the driver's perspective
    Immersive = 1,
}

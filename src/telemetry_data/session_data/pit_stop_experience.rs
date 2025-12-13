//! Defines the pit stop experience setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pit stop experience setting controlling pit stop presentation
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PitStopExperience {
    /// Automatic pit stops with minimal player involvement
    #[default]
    Automatic = 0,
    /// Broadcast-style pit stop presentation
    Broadcast = 1,
    /// Immersive pit stop experience from driver's perspective
    Immersive = 2,
}

//! Defines the collisions setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Collisions setting controlling vehicle collision behaviour
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Collisions {
    /// All collisions disabled
    #[default]
    Off = 0,
    /// Player-to-player collisions disabled, AI collisions enabled
    PlayerToPlayerOff = 1,
    /// All collisions enabled
    On = 2,
}

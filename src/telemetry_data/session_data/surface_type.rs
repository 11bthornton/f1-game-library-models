//! Defines the surface type simulation setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Surface type setting controlling track surface simulation complexity
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SurfaceType {
    /// Simplified surface simulation
    #[default]
    Simplified = 0,
    /// Realistic surface simulation with more grip variation
    Realistic = 1,
}

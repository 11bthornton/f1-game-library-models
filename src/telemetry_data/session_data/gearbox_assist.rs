//! Defines the gearbox assist settings

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Gearbox assist settings
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GearboxAssist {
    /// Manual gearbox (no assist)
    #[default]
    Manual = 1,
    /// Manual gearbox with suggested gear indicators
    SuggestedGear = 2,
    /// Fully automatic gearbox
    Automatic = 3,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

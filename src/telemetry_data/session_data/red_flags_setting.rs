//! Defines the red flags frequency setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Red flags setting controlling red flag deployment frequency
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RedFlagsSetting {
    /// Red flags disabled
    #[default]
    Off = 0,
    /// Reduced red flag deployment frequency
    Reduced = 1,
    /// Standard red flag deployment frequency
    Standard = 2,
    /// Increased red flag deployment frequency
    Increased = 3,
    // /// Unknown or not applicable
    // Unknown = 255,
}

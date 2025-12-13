//! Defines the corner cutting stringency setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Corner cutting stringency setting for track limit enforcement
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CornerCuttingStringency {
    /// Regular track limit enforcement
    #[default]
    Regular = 0,
    /// Strict track limit enforcement
    Strict = 1,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

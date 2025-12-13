//! Defines the safety car frequency setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Safety car setting controlling safety car deployment frequency
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SafetyCarSetting {
    /// Safety car disabled
    #[default]
    Off = 0,
    /// Reduced safety car deployment frequency
    Reduced = 1,
    /// Standard safety car deployment frequency
    Standard = 2,
    /// Increased safety car deployment frequency
    Increased = 3,
}

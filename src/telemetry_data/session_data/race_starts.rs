//! Defines the race starts setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Race starts setting controlling launch assistance
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RaceStarts {
    /// Manual race starts requiring player to control clutch and throttle
    #[default]
    Manual = 0,
    /// Assisted race starts with automatic launch control
    Assisted = 1,
}

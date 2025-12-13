//! Defines the tyre temperature simulation mode.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Tyre temperature simulation mode setting
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TyreTemperatureMode {
    /// Only surface temperature is simulated
    #[default]
    SurfaceOnly = 0,
    /// Both surface and carcass temperatures are simulated
    SurfaceAndCarcass = 1,
}

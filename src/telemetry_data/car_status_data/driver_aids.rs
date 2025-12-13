//! Defines driver aids settings for cars.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Anti-lock brakes settings.
///
/// These settings control whether anti-lock brakes are enabled.
#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AntiLockBrakes {
    /// Anti-lock brakes are disabled
    #[default]
    Off,
    /// Anti-lock brakes are enabled
    On,
}

/// Fuel mix settings.
///
/// These settings control the fuel mixture used by the engine.
#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum FuelMix {
    /// Lean fuel mixture (fuel-saving)
    Lean,
    /// Standard fuel mixture (balanced)
    #[default]
    Standard,
    /// Rich fuel mixture (more power)
    Rich,
    /// Maximum fuel mixture (maximum power)
    Max,
}

/// Traction control settings.
///
/// These settings control the level of traction control assistance.
#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TractionControl {
    /// Traction control is disabled
    #[default]
    Off = 0,
    /// Medium traction control
    Medium = 1,
    /// Full traction control
    Full = 2,
}

/// Pit limiter status.
///
/// These values indicate whether the pit limiter is active.
#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum PitLimiterStatus {
    /// Pit limiter is disabled
    #[default]
    Off,
    /// Pit limiter is enabled
    On,
}

//! Driver assist setting enums shared across Car Status and Session packets.

use num_enum::TryFromPrimitive;

/// Traction control setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum TractionControl {
    Off = 0,
    Medium = 1,
    Full = 2,
}

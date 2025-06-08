use serde_repr::{Deserialize_repr, Serialize_repr};

/// FIA flag types shown to the driver.
///
/// These flags indicate various race conditions and instructions to the driver.
#[derive(Deserialize_repr, Debug, Serialize_repr, Clone, Copy, PartialEq)]
#[repr(i8)]
pub enum VehicleFiaFlags {
    /// Flag status is unknown or invalid
    InvalidUnknown = -1,
    /// No flag is being shown
    None = 0,
    /// Green flag (track is clear)
    Green = 1,
    /// Blue flag (faster car approaching)
    Blue = 2,
    /// Yellow flag (danger on track)
    Yellow = 3,
    /// Red flag (session stopped)
    Red = 4,
}

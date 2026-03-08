//! Safety car type enum shared by Session and Event packets.

use num_enum::TryFromPrimitive;

/// Type of safety car on (or entering) the track.
///
/// Used in the session packet (`safety_car_status`) and in safety car events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SafetyCarType {
    None = 0,
    Full = 1,
    Virtual = 2,
    FormationLap = 3,
}

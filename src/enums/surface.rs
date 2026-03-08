//! Driving surface type enum.

use num_enum::TryFromPrimitive;

/// The driving surface under a wheel.
///
/// Values from the F1 UDP spec appendix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SurfaceType {
    Tarmac = 0,
    RumbleStrip = 1,
    Concrete = 2,
    Rock = 3,
    Gravel = 4,
    Mud = 5,
    Sand = 6,
    Grass = 7,
    Water = 8,
    Cobblestone = 9,
    Metal = 10,
    Ridged = 11,
}

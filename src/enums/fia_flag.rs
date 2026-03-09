/// FIA flag shown to a car or in a marshal zone.
///
/// The wire value is a signed byte; `-1` encodes "invalid/unknown".
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(i8)]
pub enum FiaFlag {
    Invalid = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
}

//! Platform enum shared by Lobby Info and Participants packets.

use num_enum::TryFromPrimitive;

/// The platform a participant is playing on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Platform {
    Steam = 1,
    PlayStation = 3,
    Xbox = 4,
    Origin = 6,
    Unknown = 255,
}

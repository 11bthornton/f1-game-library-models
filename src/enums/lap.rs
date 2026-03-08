//! Lap-related enums: pit status, sector, driver status, result status.

use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Sector {
    SectorOne = 0,
    SectorTwo = 1,
    SectorThree = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

/// Reason a driver's race ended. Used in Final Classification and Retirement events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ResultReason {
    Invalid = 0,
    Retired = 1,
    Finished = 2,
    TerminalDamage = 3,
    Inactive = 4,
    NotEnoughLaps = 5,
    BlackFlagged = 6,
    RedFlagged = 7,
    MechanicalFailure = 8,
    SessionSkipped = 9,
    SessionSimulated = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    Dnf = 4,
    Dsq = 5,
    NotClassified = 6,
    Retired = 7,
}

//! Defines the types of penalties that can be issued during seeions

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Types of penalties that can be issued during a race.
#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum PenaltyType {
    /// Driver must drive through the pit lane without stopping
    #[default]
    DriveThrough = 0,

    /// Driver must stop in the pit lane for a specified time
    StopGo = 1,

    /// Driver receives a grid position penalty for the next race
    GridPenalty = 2,

    /// Reminder about an outstanding penalty
    PenaltyReminder = 3,

    /// Driver receives a time penalty added to their race time
    TimePenalty = 4,

    /// Driver receives a warning for their behavior
    Warning = 5,

    /// Driver is disqualified from the session
    Disqualified = 6,

    /// Driver is removed from the formation lap
    RemovedFromFormationLap = 7,

    /// Timer for being parked too long
    ParkedTooLongTimer = 8,

    /// Penalty for violating tyre regulations
    TyreRegulations = 9,

    /// Current lap time is invalidated
    ThisLapInvalidated = 10,

    /// Current and next lap times are invalidated
    ThisAndNextLapInvalidated = 11,

    /// Current lap time is invalidated without specific reason
    ThisLapInvalidatedWithoutReason = 12,

    /// Current and next lap times are invalidated without specific reason
    ThisAndNextLapInvalidatedWithoutReason = 13,

    /// Current and previous lap times are invalidated
    ThisAndPreviousLapInvalidated = 14,

    /// Current and previous lap times are invalidated without specific reason
    ThisAndPreviousLapInvalidatedWithoutReason = 15,

    /// Driver has retired from the session
    Retired = 16,

    /// Timer for black flag (disqualification)
    BlackFlagTimer = 17,
}

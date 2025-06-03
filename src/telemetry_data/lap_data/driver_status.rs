use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Driver status.
///
/// These represent the various states a driver can be in during a session.
#[derive(Deserialize_repr, Debug, Default, Serialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DriverStatus
{
    /// Driver is in the garage
    #[serde(rename = "In Garage")]
    InGarage = 0,

    /// Driver is on a flying (timed) lap
    #[default]
    #[serde(rename = "Flying Lap")]
    FlyingLap = 1,

    /// Driver is on an in-lap (returning to pits)
    #[serde(rename = "In Lap")]
    InLap = 2,

    /// Driver is on an out-lap (leaving pits)
    #[serde(rename = "Out Lap")]
    OutLap = 3,

    /// Driver is on track but not on a timed lap
    #[serde(rename = "On Track")]
    OnTrack = 4,
}

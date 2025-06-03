use serde::Serialize;
use serde_repr::Deserialize_repr;

/// Result status.
///
/// These represent the various race result states a car can have.
#[derive(Deserialize_repr, Debug, Default, Serialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ResultStatus {
    /// Invalid result
    #[default]
    #[serde(rename = "Invalid")]
    Invalid = 0,

    /// Driver is inactive
    #[serde(rename = "Inactive")]
    Inactive = 1,

    /// Driver is active and racing
    #[serde(rename = "Active")]
    Active = 2,

    /// Driver has finished the race
    #[serde(rename = "Finished")]
    Finished = 3,

    /// Driver did not finish (DNF)
    #[serde(rename = "Did Not Finish (DNF)")]
    DNF = 4,

    /// Driver has been disqualified (DSQ)
    #[serde(rename = "Disqualified (DSQ)")]
    DSQ = 5,

    /// Driver has not been classified in the results
    #[serde(rename = "Not Classified")]
    NotClassified = 6,

    /// Driver has retired from the race
    #[serde(rename = "Retired")]
    Retired = 7,
}

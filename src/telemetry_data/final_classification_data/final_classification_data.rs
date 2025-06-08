//! Defines the `FinalClassificationData` structure for telemetry data.

use serde::{Deserialize, Serialize};

/// Final classification data for a single car.
///
/// This structure contains detailed classification information for a single car
/// at the end of a race, including position, lap times, penalties, and tyre usage.
///
/// # Fields
///
/// * `position` - Finishing position of the car
/// * `num_laps` - Number of laps completed
/// * `grid_position` - Starting grid position
/// * `points` - Championship points awarded
/// * `num_pitstops` - Number of pit stops made during the race
/// * `best_lap_time_in_ms` - Best lap time in milliseconds
/// * `total_race_time` - Total race time in milliseconds
/// * `penalties_time` - Total penalty time in seconds
/// * `num_penalties` - Number of penalties received
/// * `num_tyre_stints` - Number of tyre stints completed
/// * `tyre_stints_actual` - Actual tyre compounds used in each stint
/// * `tyre_stints_visual` - Visual tyre compounds displayed in each stint
/// * `tyre_stints_end_laps` - Lap numbers when each tyre stint ended
#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct FinalClassificationData {
    /// Finishing position of the car
    pub position: u8,
    /// Number of laps completed
    pub num_laps: u8,
    /// Starting grid position
    pub grid_position: u8,
    /// Championship points awarded
    pub points: u8,
    /// Number of pit stops made during the race
    pub num_pitstops: u8,
    /// Best lap time in milliseconds
    pub best_lap_time_in_ms: u32,
    /// Total race time in milliseconds
    pub total_race_time: u64,
    /// Total penalty time in seconds
    pub penalties_time: u8,
    /// Number of penalties received
    pub num_penalties: u8,
    /// Number of tyre stints completed
    pub num_tyre_stints: u8,
    /// Actual tyre compounds used in each stint (up to 8 stints)
    pub tyre_stints_actual: [u8; 8],
    /// Visual tyre compounds displayed in each stint (up to 8 stints)
    pub tyre_stints_visual: [u8; 8],
    /// Lap numbers when each tyre stint ended (up to 8 stints)
    pub tyre_stints_end_laps: [u8; 8],
}

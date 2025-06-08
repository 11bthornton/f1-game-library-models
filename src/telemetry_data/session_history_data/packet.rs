//! Defines the session history data packet structure.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{LapHistoryData, TyreStintHistoryData};
use crate::telemetry_data::packet_header::PacketHeader;

/// Packet containing session history data for a specific car
///
/// This structure contains historical data for a single car in the session,
/// including lap times, sector times, and tyre usage.
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketSessionHistoryData {
    /// Header information for the packet
    pub header: PacketHeader,

    /// Index of the car this data relates to
    #[serde(with = "crate::utils::u8_as_usize")]
    pub car_index: usize,

    /// Number of laps completed by this car
    pub num_laps: u8,

    /// Number of tyre stints completed by this car
    pub num_tyre_stints: u8,

    /// Lap number that achieved the best lap time
    pub best_lap_time_num: u8,

    /// Lap number that achieved the best sector 1 time
    pub best_sector_1_lap_num: u8,

    /// Lap number that achieved the best sector 2 time
    pub best_sector_2_lap_num: u8,

    /// Lap number that achieved the best sector 3 time
    pub best_sector_3_lap_num: u8,

    /// Historical lap data for all laps completed
    #[serde(with = "BigArray")]
    pub lap_history_data: [LapHistoryData; 100],

    /// Historical tyre stint data (up to 8 stints)
    pub tyre_stints_history_data: [TyreStintHistoryData; 8],
}

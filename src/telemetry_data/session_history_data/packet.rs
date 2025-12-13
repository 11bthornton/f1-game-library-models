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
    #[serde(with = "crate::utils::u8_as_usize")]
    pub best_lap_time_num: usize,

    /// Lap number that achieved the best sector 1 time
    #[serde(with = "crate::utils::u8_as_usize")]
    pub best_sector_1_lap_num: usize,

    /// Lap number that achieved the best sector 2 time
    #[serde(with = "crate::utils::u8_as_usize")]
    pub best_sector_2_lap_num: usize,

    /// Lap number that achieved the best sector 3 time
    #[serde(with = "crate::utils::u8_as_usize")]
    pub best_sector_3_lap_num: usize,

    /// Historical lap data for all laps completed
    #[serde(with = "BigArray")]
    pub lap_history_data: [LapHistoryData; 100],

    /// Historical tyre stint data (up to 8 stints)
    pub tyre_stints_history_data: [TyreStintHistoryData; 8],
}

impl Default for PacketSessionHistoryData {
    fn default() -> Self {
        Self {
            header: PacketHeader {
                packet_id: crate::telemetry_data::packet_header::PacketId::SessionHistoryPacket,
                ..PacketHeader::default()
            },
            car_index: 0,
            num_laps: 0,
            num_tyre_stints: 0,
            best_lap_time_num: 0,
            best_sector_1_lap_num: 0,
            best_sector_2_lap_num: 0,
            best_sector_3_lap_num: 0,
            lap_history_data: [LapHistoryData::default(); 100],
            tyre_stints_history_data: [TyreStintHistoryData::default(); 8],
        }
    }
}

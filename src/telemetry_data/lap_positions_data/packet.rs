use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::telemetry_data::PacketHeader;

pub const MAX_NUM_LAPS_IN_LAP_POSITIONS_HISTORY_PACKET: usize = 50;

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketLapPositionsData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Number of laps in the data
    pub num_laps: u8,

    /// Index of the lap where the data starts, 0 indexed
    pub lap_start: u8,

    /// Array of lap positions for each car (up to 22 cars)
    #[serde(with = "BigArray")]
    pub lap_positions: [CarLapHistory; 22],
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct CarLapHistory {
    /// Array of lap positions for each car (up to 50 laps)
    #[serde(with = "BigArray")]
    pub lap_positions: [u8; MAX_NUM_LAPS_IN_LAP_POSITIONS_HISTORY_PACKET],
}

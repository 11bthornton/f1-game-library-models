use serde::{
    Deserialize,
    Serialize,
};

/// Historical data for a single lap
///
/// Contains timing information for a lap, including overall lap time
/// and individual sector times
#[derive(Deserialize, Debug, Serialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct LapHistoryData
{
    /// Lap time in milliseconds
    pub lap_time_in_ms: u32,
    /// Sector 1 time in milliseconds
    pub sector_1_time_in_ms: u16,
    /// Sector 1 whole minute part
    pub sector_1_time_minutes: u8,
    /// Sector 2 time in milliseconds
    pub sector_2_time_in_ms: u16,
    /// Sector 2 whole minute part
    pub sector_2_time_minutes: u8,
    /// Sector 3 time in milliseconds
    pub sector_3_time_in_ms: u16,
    /// Sector 3 whole minute part
    pub sector_3_time_minutes: u8,
    /// Bit flags indicating lap validity (0x01 = lap valid, 0x02 = sector 1 valid, 0x04 = sector 2
    /// valid, 0x08 = sector 3 valid)
    pub lap_valid_bit_flags: u8,
}

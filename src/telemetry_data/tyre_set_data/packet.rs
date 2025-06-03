use serde::{Deserialize, Serialize};

use super::tyre_set::TyreSetData;
use crate::telemetry_data::packet_header::PacketHeader;

/// Packet containing tyre set data for a car.
///
/// This structure contains information about all tyre sets available to a car.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `car_idx` - Index of the car this data relates to
/// * `tyre_set_data` - Array of tyre set data (13 dry + 7 wet = 20 total)
/// * `fitted_idx` - Index into array of the currently fitted tyre set
#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct PacketTyreSetsData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Index of the car this data relates to
    #[serde(with = "crate::utils::u8_as_usize")]
    pub car_idx: usize,
    /// Array of tyre set data (13 dry + 7 wet = 20 total)
    pub tyre_set_data: [TyreSetData; 20],
    /// Index into array of the currently fitted tyre set
    #[serde(with = "crate::utils::u8_as_usize")]
    pub fitted_idx: usize,
}

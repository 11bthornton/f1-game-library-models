use serde::{Deserialize, Serialize};

use super::car_status_data::CarStatusData;
use crate::telemetry_data::packet_header::PacketHeader;

/// Packet containing status data for all cars in the session.
///
/// This structure contains status data for all cars in the session.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `car_status_data` - Array of status data for each car (up to 22 cars)
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]
pub struct PacketCarStatusData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Array of status data for each car (up to 22 cars)
    pub car_status_data: [CarStatusData; 22],
}

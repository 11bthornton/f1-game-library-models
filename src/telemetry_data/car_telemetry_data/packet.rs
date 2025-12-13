//! Defines the `PacketCarTelemetryData` structure, which contains telemetry data for all cars in the session.

use serde::{Deserialize, Serialize};

use super::car_telemetry::CarTelemetryData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

/// Packet containing telemetry data for all cars in the session.
///
/// This structure contains telemetry data for all cars in the session, as well as
/// information about the Multi-Function Display (MFD) and suggested gear.
///
/// # Fields
///
/// * `m_header` - Header information for the packet
/// * `telemetry_data` - Array of telemetry data for each car (up to 22 cars)
/// * `mfd_panel_index` - Index of the MFD panel being displayed
/// * `mfd_panel_index_secondary_player` - Index of the MFD panel for the secondary player
/// * `suggested_gear` - Suggested gear for the player (-1 if no suggestion)
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketCarTelemetryData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Array of telemetry data for each car (exactly 22 cars)
    pub car_telemetry_data: [CarTelemetryData; MAX_CARS_IN_SESSION],
    /// Index of the MFD panel being displayed
    pub mfd_panel_index: u8,
    /// Index of the MFD panel for the secondary player
    pub mfd_panel_index_secondary_player: u8,
    /// Suggested gear for the player (-1 if no suggestion)
    pub suggested_gear: i8,
}

impl Default for PacketCarTelemetryData {
    fn default() -> Self {
        Self {
            header: PacketHeader {
                packet_id: crate::telemetry_data::PacketId::CarTelemetryPacket,
                ..PacketHeader::default()
            },
            car_telemetry_data: [CarTelemetryData::default(); MAX_CARS_IN_SESSION],
            mfd_panel_index: 0,
            mfd_panel_index_secondary_player: 0,
            suggested_gear: -1,
        }
    }
}
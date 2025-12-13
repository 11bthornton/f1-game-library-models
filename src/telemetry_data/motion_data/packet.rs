//! Defines the `PacketMotionData` struct.

use serde::{Deserialize, Serialize};

use super::CarMotionData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

/// Motion data packet containing car physics information.
///
/// This packet details car motion and physics data for all cars in the race.
/// It includes arrays of motion data for each car, as well as additional data
/// for the player's car.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PacketCarMotionData {
    /// Header information for the packet
    pub header: PacketHeader,

    /// Motion data for all cars on track
    pub car_motion_data: [CarMotionData; MAX_CARS_IN_SESSION],
}

impl Default for PacketCarMotionData {
    fn default() -> Self {
        Self {
            header: PacketHeader {
                packet_id: crate::telemetry_data::PacketId::CarMotionPacket,
                ..PacketHeader::default()
            },
            car_motion_data: [CarMotionData::default(); MAX_CARS_IN_SESSION],
        }
    }
}
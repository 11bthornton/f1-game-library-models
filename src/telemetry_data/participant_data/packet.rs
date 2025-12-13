//! Defines the PacketParticipantData structure for telemetry data.

use serde::{Deserialize, Serialize};

use super::participant_data::ParticipantData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

#[derive(Deserialize, Debug, Serialize, Copy, Clone, PartialEq)]
pub struct PacketParticipantData {
    /// Packet header containing identifying information
    pub header: PacketHeader,

    /// Number of active cars in the data
    pub num_active_cars: u8,

    /// Array of participant data for all cars
    pub participant_data: [ParticipantData; MAX_CARS_IN_SESSION],
}

impl Default for PacketParticipantData {
    fn default() -> Self {
        let header = PacketHeader {
            packet_id: crate::telemetry_data::packet_header::PacketId::ParticipantsPacket,
            ..Default::default()
        };

        Self {
            header,
            num_active_cars: 0,
            participant_data: [ParticipantData::default(); MAX_CARS_IN_SESSION],
        }
    }
}
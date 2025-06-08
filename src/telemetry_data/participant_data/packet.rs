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
    pub participants: [ParticipantData; MAX_CARS_IN_SESSION],
}

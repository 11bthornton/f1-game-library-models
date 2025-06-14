//! Defines the car damage data packet structure.
//!
//! This module contains the definition of the `PacketCarDamageData` structure,
//! which represents the damage data for all cars in a racing session. The packet
//! includes a header and an array of damage data for each car, up to a maximum
//! number of cars defined by `MAX_CARS_IN_SESSION`.

use serde::{Deserialize, Serialize};

use super::CarDamageData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

/// Packet containing damage data for all cars in the session.
///
/// This structure contains damage data for all cars in the session.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `car_damage_data` - Array of damage data for each car (up to 22 cars)
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketCarDamageData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Array of damage data for each car (up to 22 cars)
    pub car_damage_data: [CarDamageData; MAX_CARS_IN_SESSION],
}

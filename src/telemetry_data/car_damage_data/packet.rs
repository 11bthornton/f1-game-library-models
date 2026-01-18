//! Defines the car damage data packet structure.
//!
//! This module contains the definition of the `PacketCarDamageData` structure,
//! which represents the damage data for all cars in a racing session. The packet
//! includes a header and an array of damage data for each car, up to a maximum
//! number of cars defined by `MAX_CARS_IN_SESSION`.

use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::CarDamageData;
use crate::{FormulaOnePacket, constants::{CAR_DAMAGE_DATA_PACKET_SIZE, MAX_CARS_IN_SESSION}, telemetry_data::Formula};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CarDamageArray(
    pub [CarDamageData; MAX_CARS_IN_SESSION]
);

impl Deref for CarDamageArray {
    type Target = [CarDamageData; MAX_CARS_IN_SESSION ];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl crate::Pod for CarDamageArray {}

impl FormulaOnePacket for CarDamageArray {
    const PACKET_SIZE: usize = CAR_DAMAGE_DATA_PACKET_SIZE;

    fn fix_endianness(&mut self) {
        // TODO
    }
}
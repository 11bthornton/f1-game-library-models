//! Defines the car setup data packet structure.
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use crate::{constants::{CAR_SETUP_PACKET_SIZE, MAX_CARS_IN_SESSION}, telemetry_data::packet_header::PacketHeader};

pub use car_setup_data::CarSetupData;

mod car_setup_data;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CarSetupArray(
    pub [CarSetupData; MAX_CARS_IN_SESSION]
);

impl Deref for CarSetupArray {
    type Target = [CarSetupData; MAX_CARS_IN_SESSION ];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl crate::Pod for CarSetupArray {}

impl crate::telemetry_data::FormulaOnePacket for CarSetupArray {
    const PACKET_SIZE: usize = CAR_SETUP_PACKET_SIZE;

    fn fix_endianness(&mut self) {
        // TODO
    }
}

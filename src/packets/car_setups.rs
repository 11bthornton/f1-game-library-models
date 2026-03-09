//! Wire-format structures for the Car Setups packet (packet id 5).
//!
//! Spec: `PacketCarSetupData` — 1133 bytes total.
//! Per-car struct: `CarSetupData` — 50 bytes.
//!
//! Note: tyre pressures and fuel load are `f32` in the spec. The v1
//! implementation incorrectly stores these as `u8`.

use std::mem::size_of;

use crate::constants::{CAR_SETUP_PACKET_SIZE, MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE};

use super::super::{Packet, WheelData, endian::FixEndianness, macros::wire_field_accessors};

/// Wire-format setup data for a single car.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarSetupData {
    pub front_wing: u8,
    pub rear_wing: u8,
    /// Differential adjustment on throttle (percentage).
    pub on_throttle: u8,
    /// Differential adjustment off throttle (percentage).
    pub off_throttle: u8,
    front_camber: f32,
    rear_camber: f32,
    front_toe: f32,
    rear_toe: f32,
    pub front_suspension: u8,
    pub rear_suspension: u8,
    pub front_anti_roll_bar: u8,
    pub rear_anti_roll_bar: u8,
    pub front_suspension_height: u8,
    pub rear_suspension_height: u8,
    pub brake_pressure: u8,
    pub brake_bias: u8,
    pub engine_braking: u8,
    /// Tyre pressures in PSI — wire order: RL, RR, FL, FR.
    tyre_pressure: WheelData<f32>,
    pub ballast: u8,
    fuel_load: f32,
}

impl CarSetupData {
    wire_field_accessors!(
        front_camber: f32,
        rear_camber: f32,
        front_toe: f32,
        rear_toe: f32,
        tyre_pressure: WheelData<f32>,
        fuel_load: f32,
    );
}

const _: () = assert!(
    size_of::<CarSetupData>() == (CAR_SETUP_PACKET_SIZE - PACKET_HEADER_SIZE - size_of::<f32>()) / MAX_CARS_IN_SESSION
);

impl FixEndianness for CarSetupData {
    fn fix_endianness(self) -> Self {
        Self {
            front_camber: self.front_camber.fix_endianness(),
            rear_camber: self.rear_camber.fix_endianness(),
            front_toe: self.front_toe.fix_endianness(),
            rear_toe: self.rear_toe.fix_endianness(),
            tyre_pressure: self.tyre_pressure.fix_endianness(),
            fuel_load: self.fuel_load.fix_endianness(),
            // u8 fields need no swapping
            ..self
        }
    }
}

/// Payload of the car setups packet.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarSetupPayload {
    pub car_setups: [CarSetupData; MAX_CARS_IN_SESSION],
    /// Front wing value after next pit stop (player car only).
    next_front_wing_value: f32,
}

impl CarSetupPayload {
    wire_field_accessors!(next_front_wing_value: f32);
}

const _: () = assert!(size_of::<CarSetupPayload>() == CAR_SETUP_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format car setups packet for all cars in the session.
///
/// `header.packet_id` will be `5` for this packet type.
pub type PacketCarSetups = Packet<CarSetupPayload>;

const _: () = assert!(size_of::<PacketCarSetups>() == CAR_SETUP_PACKET_SIZE);

impl FixEndianness for CarSetupPayload {
    fn fix_endianness(self) -> Self {
        Self {
            car_setups: self.car_setups.fix_endianness(),
            next_front_wing_value: self.next_front_wing_value.fix_endianness(),
        }
    }
}

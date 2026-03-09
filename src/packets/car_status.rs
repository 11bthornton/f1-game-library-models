//! Wire-format structures for the Car Status packet (packet id 7).
//!
//! Spec: `PacketCarStatusData` — 1239 bytes total.
//! Per-car struct: `CarStatusData` — 55 bytes.

use std::mem::size_of;

use crate::constants::{CAR_STATUS_PACKET_SIZE, MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{ActualTyreCompound, FiaFlag, TractionControl, VisualTyreCompound},
    macros::{wire_enum_accessors, wire_field_accessors, wire_flag_accessors, wire_i8_enum_accessors},
};

/// ERS energy deployment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum ErsDeployMode {
    None = 0,
    Medium = 1,
    Hotlap = 2,
    Overtake = 3,
}

/// Fuel mixture setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

/// Wire-format car status data for a single car.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarStatusData {
    traction_control: u8,
    anti_lock_brakes: u8,
    fuel_mix: u8,
    /// Front brake bias (percentage).
    pub front_brake_bias: u8,
    pit_limiter_status: u8,
    /// Current fuel mass.
    fuel_in_tank: f32,
    /// Fuel capacity.
    fuel_capacity: f32,
    /// Fuel remaining in terms of laps (MFD value).
    fuel_remaining_laps: f32,
    /// Rev limiter RPM.
    max_rpm: u16,
    /// Idle RPM.
    idle_rpm: u16,
    /// Maximum number of gears.
    pub max_gears: u8,
    drs_allowed: u8,
    /// Distance at which DRS becomes available (metres). `0` = not available.
    drs_activation_distance: u16,
    actual_tyre_compound: u8,
    visual_tyre_compound: u8,
    /// Age of the current tyre set in laps.
    pub tyres_age_laps: u8,
    vehicle_fia_flags: i8,
    /// Engine power output of ICE (W).
    engine_power_ice: f32,
    /// Engine power output of MGU-K (W).
    engine_power_mguk: f32,
    /// ERS energy store (Joules).
    ers_store_energy: f32,
    ers_deploy_mode: u8,
    /// ERS energy harvested this lap by MGU-K (Joules).
    ers_harvested_this_lap_mguk: f32,
    /// ERS energy harvested this lap by MGU-H (Joules).
    ers_harvested_this_lap_mguh: f32,
    /// ERS energy deployed this lap (Joules).
    ers_deployed_this_lap: f32,
    network_paused: u8,
}

const _: () = assert!(size_of::<CarStatusData>() == 55);

impl CarStatusData {
    wire_flag_accessors!(anti_lock_brakes, pit_limiter_status, drs_allowed, network_paused);
    wire_i8_enum_accessors!(vehicle_fia_flags => FiaFlag);

    wire_enum_accessors!(
        traction_control      => TractionControl,
        fuel_mix              => FuelMix,
        actual_tyre_compound  => ActualTyreCompound,
        visual_tyre_compound  => VisualTyreCompound,
        ers_deploy_mode       => ErsDeployMode,
    );

    wire_field_accessors!(
        fuel_in_tank: f32,
        fuel_capacity: f32,
        fuel_remaining_laps: f32,
        max_rpm: u16,
        idle_rpm: u16,
        drs_activation_distance: u16,
        engine_power_ice: f32,
        engine_power_mguk: f32,
        ers_store_energy: f32,
        ers_harvested_this_lap_mguk: f32,
        ers_harvested_this_lap_mguh: f32,
        ers_deployed_this_lap: f32,
    );
}

impl FixEndianness for CarStatusData {
    fn fix_endianness(self) -> Self {
        Self {
            fuel_in_tank: self.fuel_in_tank.fix_endianness(),
            fuel_capacity: self.fuel_capacity.fix_endianness(),
            fuel_remaining_laps: self.fuel_remaining_laps.fix_endianness(),
            max_rpm: self.max_rpm.fix_endianness(),
            idle_rpm: self.idle_rpm.fix_endianness(),
            drs_activation_distance: self.drs_activation_distance.fix_endianness(),
            engine_power_ice: self.engine_power_ice.fix_endianness(),
            engine_power_mguk: self.engine_power_mguk.fix_endianness(),
            ers_store_energy: self.ers_store_energy.fix_endianness(),
            ers_harvested_this_lap_mguk: self.ers_harvested_this_lap_mguk.fix_endianness(),
            ers_harvested_this_lap_mguh: self.ers_harvested_this_lap_mguh.fix_endianness(),
            ers_deployed_this_lap: self.ers_deployed_this_lap.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format car status payload for all cars.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarStatusPayload {
    pub car_status_data: [CarStatusData; MAX_CARS_IN_SESSION],
}

const _: () = assert!(size_of::<CarStatusPayload>() == CAR_STATUS_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format car status packet for all cars in the session.
///
/// `header.packet_id` will be `7` for this packet type.
pub type PacketCarStatus = Packet<CarStatusPayload>;

const _: () = assert!(size_of::<PacketCarStatus>() == CAR_STATUS_PACKET_SIZE);

impl FixEndianness for CarStatusPayload {
    fn fix_endianness(self) -> Self {
        Self {
            car_status_data: self.car_status_data.fix_endianness(),
        }
    }
}

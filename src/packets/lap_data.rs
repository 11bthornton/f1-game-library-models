//! Wire-format structures for the Lap Data packet (packet id 2).
//!
//! Spec: `PacketLapData` — 1285 bytes total.
//! Per-car struct: `LapData` — 57 bytes.

use std::mem::size_of;

use crate::constants::{LAP_DATA_PACKET_SIZE, MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{DriverStatus, PitStatus, ResultStatus, Sector},
    macros::{wire_enum_accessors, wire_flag_accessors, wire_index_accessors},
};

/// Wire-format lap data for a single car.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,
    pub current_lap_time_in_ms: u32,
    pub sector1_time_in_ms: u16,
    pub sector1_time_minutes: u8,
    pub sector2_time_in_ms: u16,
    pub sector2_time_minutes: u8,
    pub delta_to_car_in_front_in_ms: u16,
    pub delta_to_car_in_front_minutes: u8,
    pub delta_to_race_leader_in_ms: u16,
    pub delta_to_race_leader_minutes: u8,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pit_status: u8,
    pub num_pit_stops: u8,
    sector: u8,
    current_lap_invalid: u8,
    pub penalties: u8,
    pub total_warnings: u8,
    pub corner_cutting_warnings: u8,
    pub num_unserved_drive_through_pens: u8,
    pub num_unserved_stop_go_pens: u8,
    pub grid_position: u8,
    driver_status: u8,
    result_status: u8,
    pit_lane_timer_active: u8,
    pub pit_lane_time_in_lane_in_ms: u16,
    pub pit_stop_timer_in_ms: u16,
    pit_stop_should_serve_pen: u8,
    pub speed_trap_fastest_speed: f32,
    pub speed_trap_fastest_lap: u8,
}

const _: () = assert!(size_of::<LapData>() == 57);

impl LapData {
    wire_enum_accessors!(
        pit_status    => PitStatus,
        sector        => Sector,
        driver_status => DriverStatus,
        result_status => ResultStatus,
    );
    wire_flag_accessors!(current_lap_invalid, pit_lane_timer_active, pit_stop_should_serve_pen);
}

impl FixEndianness for LapData {
    fn fix_endianness(self) -> Self {
        Self {
            last_lap_time_in_ms: self.last_lap_time_in_ms.fix_endianness(),
            current_lap_time_in_ms: self.current_lap_time_in_ms.fix_endianness(),
            sector1_time_in_ms: self.sector1_time_in_ms.fix_endianness(),
            sector2_time_in_ms: self.sector2_time_in_ms.fix_endianness(),
            delta_to_car_in_front_in_ms: self.delta_to_car_in_front_in_ms.fix_endianness(),
            delta_to_race_leader_in_ms: self.delta_to_race_leader_in_ms.fix_endianness(),
            lap_distance: self.lap_distance.fix_endianness(),
            total_distance: self.total_distance.fix_endianness(),
            safety_car_delta: self.safety_car_delta.fix_endianness(),
            pit_lane_time_in_lane_in_ms: self.pit_lane_time_in_lane_in_ms.fix_endianness(),
            pit_stop_timer_in_ms: self.pit_stop_timer_in_ms.fix_endianness(),
            speed_trap_fastest_speed: self.speed_trap_fastest_speed.fix_endianness(),
            ..self
        }
    }
}

/// Payload of the lap data packet.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LapDataPayload {
    pub lap_data: [LapData; MAX_CARS_IN_SESSION],
    /// Index of the time trial personal best car (255 if not set).
    time_trial_pb_car_idx: u8,
    /// Index of the time trial rival car (255 if not set).
    time_trial_rival_car_idx: u8,
}

const _: () = assert!(size_of::<LapDataPayload>() == LAP_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format lap data packet for all cars in the session.
///
/// `header.packet_id` will be `2` for this packet type.
pub type PacketLapData = Packet<LapDataPayload>;

const _: () = assert!(size_of::<PacketLapData>() == LAP_DATA_PACKET_SIZE);

impl LapDataPayload {
    wire_index_accessors!(time_trial_pb_car_idx, time_trial_rival_car_idx);
}

impl FixEndianness for LapDataPayload {
    fn fix_endianness(self) -> Self {
        Self {
            lap_data: self.lap_data.fix_endianness(),
            ..self
        }
    }
}

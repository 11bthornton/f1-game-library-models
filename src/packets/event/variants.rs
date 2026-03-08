//! Per-event payload structs.
//!
//! Each struct maps to one variant of the C++ `EventDataDetails` union.
//! They are read from raw bytes via `ptr::read_unaligned` inside
//! [`super::EventPayload::event_data`].

use super::super::super::{
    endian::FixEndianness,
    macros::{wire_enum_accessors, wire_index_accessors},
};
use crate::enums::{ResultReason, SafetyCarType};

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FastestLap {
    vehicle_idx: u8,
    /// Lap time in seconds.
    pub lap_time: f32,
}

impl FastestLap {
    wire_index_accessors!(vehicle_idx);
}

impl FixEndianness for FastestLap {
    fn fix_endianness(self) -> Self {
        Self {
            lap_time: self.lap_time.fix_endianness(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Retirement {
    vehicle_idx: u8,
    reason: u8,
}

impl Retirement {
    wire_index_accessors!(vehicle_idx);
    wire_enum_accessors!(reason => ResultReason);
}

impl FixEndianness for Retirement {
    fn fix_endianness(self) -> Self {
        self
    }
}

/// DRS disabled reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum DrsDisabledReason {
    WetTrack = 0,
    SafetyCarDeployed = 1,
    RedFlag = 2,
    MinLapNotReached = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DrsDisabled {
    reason: u8,
}

impl DrsDisabled {
    wire_enum_accessors!(reason => DrsDisabledReason);
}

impl FixEndianness for DrsDisabled {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TeamMateInPits {
    vehicle_idx: u8,
}

impl TeamMateInPits {
    wire_index_accessors!(vehicle_idx);
}

impl FixEndianness for TeamMateInPits {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct RaceWinner {
    vehicle_idx: u8,
}

impl RaceWinner {
    wire_index_accessors!(vehicle_idx);
}

impl FixEndianness for RaceWinner {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Penalty {
    /// Penalty type (see appendix).
    pub penalty_type: u8,
    /// Infringement type (see appendix).
    pub infringement_type: u8,
    vehicle_idx: u8,
    other_vehicle_idx: u8,
    /// Time gained or time spent doing the action (seconds).
    pub time: u8,
    /// Lap the penalty occurred on.
    pub lap_num: u8,
    /// Number of places gained.
    pub places_gained: u8,
}

impl Penalty {
    wire_index_accessors!(vehicle_idx, other_vehicle_idx);
}

impl FixEndianness for Penalty {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SpeedTrap {
    vehicle_idx: u8,
    /// Top speed achieved (km/h).
    pub speed: f32,
    pub is_overall_fastest_in_session: u8,
    pub is_driver_fastest_in_session: u8,
    fastest_vehicle_idx_in_session: u8,
    /// Speed of the overall fastest vehicle in the session (km/h).
    pub fastest_speed_in_session: f32,
}

impl SpeedTrap {
    wire_index_accessors!(vehicle_idx, fastest_vehicle_idx_in_session);
}

impl FixEndianness for SpeedTrap {
    fn fix_endianness(self) -> Self {
        Self {
            speed: self.speed.fix_endianness(),
            fastest_speed_in_session: self.fastest_speed_in_session.fix_endianness(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct StartLights {
    pub num_lights: u8,
}

impl FixEndianness for StartLights {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DriveThroughPenaltyServed {
    vehicle_idx: u8,
}

impl DriveThroughPenaltyServed {
    wire_index_accessors!(vehicle_idx);
}

impl FixEndianness for DriveThroughPenaltyServed {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct StopGoPenaltyServed {
    vehicle_idx: u8,
    /// Time spent serving the stop-go (seconds).
    pub stop_time: f32,
}

impl StopGoPenaltyServed {
    wire_index_accessors!(vehicle_idx);
}

impl FixEndianness for StopGoPenaltyServed {
    fn fix_endianness(self) -> Self {
        Self {
            stop_time: self.stop_time.fix_endianness(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Flashback {
    pub flashback_frame_identifier: u32,
    pub flashback_session_time: f32,
}

impl FixEndianness for Flashback {
    fn fix_endianness(self) -> Self {
        Self {
            flashback_frame_identifier: self.flashback_frame_identifier.fix_endianness(),
            flashback_session_time: self.flashback_session_time.fix_endianness(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Buttons {
    /// Bitfield of currently pressed buttons (see appendix).
    pub button_status: u32,
}

impl FixEndianness for Buttons {
    fn fix_endianness(self) -> Self {
        Self {
            button_status: self.button_status.fix_endianness(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Overtake {
    overtaking_vehicle_idx: u8,
    being_overtaken_vehicle_idx: u8,
}

impl Overtake {
    wire_index_accessors!(overtaking_vehicle_idx, being_overtaken_vehicle_idx);
}

impl FixEndianness for Overtake {
    fn fix_endianness(self) -> Self {
        self
    }
}

/// Safety car event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum SafetyCarEventType {
    Deployed = 0,
    Returning = 1,
    Returned = 2,
    ResumeRace = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SafetyCar {
    safety_car_type: u8,
    event_type: u8,
}

impl SafetyCar {
    wire_enum_accessors!(
        safety_car_type => SafetyCarType,
        event_type      => SafetyCarEventType,
    );
}

impl FixEndianness for SafetyCar {
    fn fix_endianness(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Collision {
    vehicle1_idx: u8,
    vehicle2_idx: u8,
}

impl Collision {
    wire_index_accessors!(vehicle1_idx, vehicle2_idx);
}

impl FixEndianness for Collision {
    fn fix_endianness(self) -> Self {
        self
    }
}

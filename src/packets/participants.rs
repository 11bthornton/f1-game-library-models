//! Wire-format structures for the Participants packet (packet id 4).
//!
//! Spec: `PacketParticipantsData` — 1284 bytes total.
//! Per-car struct: `ParticipantData` — 57 bytes.

use std::ffi::CStr;
use std::mem::size_of;

use crate::constants::{MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE, PARTICIPANT_PACKET_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{Platform, Team},
    macros::{wire_enum_accessors, wire_flag_accessors},
};

/// RGB livery colour.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LiveryColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

const _: () = assert!(size_of::<LiveryColour>() == 3);

impl FixEndianness for LiveryColour {
    fn fix_endianness(self) -> Self {
        // All u8 — no swapping needed.
        self
    }
}

/// Wire-format participant data for a single car.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ParticipantData {
    ai_controlled: u8,
    /// Driver id (see appendix). `255` for a network human player.
    pub driver_id: u8,
    /// Network id — unique identifier for network players.
    pub network_id: u8,
    team_id: u8,
    my_team: u8,
    /// Race number of the car.
    pub race_number: u8,
    /// Nationality of the driver.
    pub nationality: u8,
    name: [u8; 32],
    your_telemetry: u8,
    show_online_names: u8,
    /// F1 World tech level.
    pub tech_level: u16,
    platform: u8,
    /// Number of valid livery colours for this car.
    pub num_colours: u8,
    pub livery_colours: [LiveryColour; 4],
}

const _: () = assert!(size_of::<ParticipantData>() == 57);

impl ParticipantData {
    wire_flag_accessors!(ai_controlled, my_team, your_telemetry, show_online_names);

    wire_enum_accessors!(
        team_id  => Team,
        platform => Platform,
    );

    /// Participant name as a UTF-8 string slice.
    ///
    /// Returns `None` if the bytes are not valid UTF-8 or contain no null terminator.
    pub fn name(&self) -> Option<&str> {
        CStr::from_bytes_until_nul(&self.name)
            .ok()
            .and_then(|s| s.to_str().ok())
    }
}

impl FixEndianness for ParticipantData {
    fn fix_endianness(self) -> Self {
        Self {
            tech_level: self.tech_level.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format participants payload.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ParticipantsPayload {
    /// Number of active cars in the data.
    pub num_active_cars: u8,
    pub participants: [ParticipantData; MAX_CARS_IN_SESSION],
}

const _: () = assert!(size_of::<ParticipantsPayload>() == PARTICIPANT_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format participants packet for all cars in the session.
///
/// `header.packet_id` will be `4` for this packet type.
pub type PacketParticipants = Packet<ParticipantsPayload>;

const _: () = assert!(size_of::<PacketParticipants>() == PARTICIPANT_PACKET_SIZE);

impl FixEndianness for ParticipantsPayload {
    fn fix_endianness(self) -> Self {
        Self {
            participants: self.participants.fix_endianness(),
            ..self
        }
    }
}

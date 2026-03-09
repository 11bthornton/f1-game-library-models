//! Wire-format structures for the Lobby Info packet (packet id 9).
//!
//! Spec: `PacketLobbyInfoData` — 954 bytes total.
//! Per-player struct: `LobbyInfoData` — 42 bytes.

use std::ffi::CStr;
use std::mem::size_of;

use crate::constants::{LOBBY_INFO_DATA_PACKET_SIZE, MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{Nationality, Platform, Team},
    macros::{wire_enum_accessors, wire_field_accessors, wire_flag_accessors},
};

/// Ready status of a lobby participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum ReadyStatus {
    NotReady = 0,
    Ready = 1,
    Spectating = 2,
}

/// Wire-format lobby info for a single participant.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LobbyInfoData {
    ai_controlled: u8,
    team_id: u8,
    nationality: u8,
    platform: u8,
    name: [u8; 32],
    /// Car number of the player.
    pub car_number: u8,
    your_telemetry: u8,
    show_online_names: u8,
    /// F1 World tech level.
    tech_level: u16,
    ready_status: u8,
}

const _: () = assert!(size_of::<LobbyInfoData>() == 42);

impl LobbyInfoData {
    wire_flag_accessors!(ai_controlled, your_telemetry, show_online_names);

    wire_enum_accessors!(
        team_id      => Team,
        platform     => Platform,
        nationality  => Nationality,
        ready_status => ReadyStatus,
    );

    wire_field_accessors!(tech_level: u16);

    /// Participant name as a UTF-8 string slice.
    ///
    /// Returns `None` if the bytes are not valid UTF-8 or contain no null terminator.
    pub fn name(&self) -> Option<&str> {
        CStr::from_bytes_until_nul(&self.name)
            .ok()
            .and_then(|s| s.to_str().ok())
    }
}

impl FixEndianness for LobbyInfoData {
    fn fix_endianness(self) -> Self {
        Self {
            tech_level: self.tech_level.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format lobby info payload.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LobbyInfoPayload {
    /// Number of players currently in the lobby.
    pub num_players: u8,
    pub lobby_players: [LobbyInfoData; MAX_CARS_IN_SESSION],
}

const _: () = assert!(size_of::<LobbyInfoPayload>() == LOBBY_INFO_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format lobby info packet.
///
/// `header.packet_id` will be `9` for this packet type.
pub type PacketLobbyInfo = Packet<LobbyInfoPayload>;

const _: () = assert!(size_of::<PacketLobbyInfo>() == LOBBY_INFO_DATA_PACKET_SIZE);

impl FixEndianness for LobbyInfoPayload {
    fn fix_endianness(self) -> Self {
        Self {
            lobby_players: self.lobby_players.fix_endianness(),
            ..self
        }
    }
}

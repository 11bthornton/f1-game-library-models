/// Header information for F1 telemetry data packets.
///
/// This module defines the common header structure that is present in all F1 telemetry
/// data packets. The header contains metadata about the packet, such as its format,
/// the game version, session information, and more.
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use crate::{FormulaOnePacket, Pod, constants::HEADER_SIZE};

/// Raw POD header structure - direct wire format representation
///
/// This is the exact binary layout received from UDP packets.
/// All multi-byte fields are in little-endian format as sent by the game.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Default)]
#[repr(C, packed)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub game_year: u8,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub overall_frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

unsafe impl Pod for PacketHeader {}

impl FormulaOnePacket for PacketHeader {
    const PACKET_SIZE: usize = HEADER_SIZE;

    /// Fix endianness of multi-byte fields (convert from little-endian to native)
    ///
    /// This is a no-op on little-endian systems (x86, x64, ARM-LE).
    /// On big-endian systems, it swaps bytes to correct endianness.
     fn fix_endianness(&mut self) {
        self.packet_format = u16::from_le(self.packet_format);
        self.session_uid = u64::from_le(self.session_uid);
        self.session_time = f32::from_bits(u32::from_le(self.session_time.to_bits()));
        self.frame_identifier = u32::from_le(self.frame_identifier);
        self.overall_frame_identifier = u32::from_le(self.overall_frame_identifier);
    }
}

/// Packet type identifier
#[derive(
    Serialize_repr,
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u8)]
pub enum PacketId {
    MotionPacket = 0,
    SessionPacket = 1,
    LapDataPacket = 2,
    EventPacket = 3,
    ParticipantsPacket = 4,
    CarSetupsPacket = 5,
    CarTelemetryPacket = 6,
    CarStatusPacket = 7,
    FinalClassificationPacket = 8,
    LobbyInfoPacket = 9,
    CarDamagePacket = 10,
    SessionHistoryPacket = 11,
    TyreSetsPacket = 12,
    MotionExPacket = 13,
    TimeTrialPacket = 14,
    LapPositionPacket = 15,
    #[default]
    None = 16,
}

/// Header structure for all F1 telemetry data packets.
///
/// Every telemetry packet begins with this header, which provides metadata about
/// the packet and the game session. This information can be used to identify the
/// packet type, the game version, and the session context.
///
impl PacketHeader {
    /// Format of the packet (game-specific)
    pub fn packet_format(&self) -> u16 {
        self.packet_format
    }

    /// Game year (e.g., 23 for F1 2023)
    pub fn game_year(&self) -> u8 {
        self.game_year
    }

    /// Major version of the game
    pub fn game_major_version(&self) -> u8 {
        self.game_major_version
    }

    /// Minor version of the game
    pub fn game_minor_version(&self) -> u8 {
        self.game_minor_version
    }

    /// Version of the packet format
    pub fn packet_version(&self) -> u8 {
        self.packet_version
    }

    /// Identifier for the packet type
    pub fn packet_id(&self) -> PacketId {
        PacketId::try_from(self.packet_id).unwrap_or_default()
    }

    /// Get the raw packet ID value (useful for debugging invalid IDs)
    pub fn packet_id_raw(&self) -> u8 {
        self.packet_id
    }

    /// Unique identifier for the session
    pub fn session_uid(&self) -> u64 {
        self.session_uid
    }

    /// Session timestamp
    pub fn session_time(&self) -> f32 {
        self.session_time
    }

    /// Identifier for the frame
    pub fn frame_identifier(&self) -> u32 {
        self.frame_identifier
    }

    /// Overall identifier for the frame
    pub fn overall_frame_identifier(&self) -> u32 {
        self.overall_frame_identifier
    }

    /// Index of the player's car in the data arrays
    pub fn player_car_index(&self) -> usize {
        self.player_car_index as usize
    }

    /// Index of the secondary player's car (split-screen)
    pub fn secondary_player_car_index(&self) -> usize {
        self.secondary_player_car_index as usize
    }
}
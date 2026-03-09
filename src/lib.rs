//! # F1 Game Library Models 25
//!
//! Data types and zero-copy deserialization for the UDP telemetry packets emitted
//! by EA Sports F1 25. Based on the
//! [official UDP specification](https://forums.ea.com/blog/f1-games-game-info-hub-en/ea-sports%E2%84%A2-f1%C2%AE25-udp-specification/12187347).
//! Previous game versions are not supported.
//!
//! ## Quick start
//! ```no_run
//! use f1_game_library_models_25::parse;
//!
//! fn handle(buf: &[u8]) {
//!     match parse(buf) {
//!         Ok(packet) => println!("{packet:?}"),
//!         Err(e) => eprintln!("parse error: {e}"),
//!     }
//! }
//! ```
//!
//! ## UDP client
//! Enable the `client` feature for a ready-made async listener:
//! ```no_run
//! use f1_game_library_models_25::client::{TelemetryClient, HandlePacket, TelemetryControl};
//! use f1_game_library_models_25::packets::lap_data::PacketLapData;
//!
//! struct MyHandler;
//!
//! impl HandlePacket for MyHandler {
//!     async fn handle_lap_data(&mut self, p: PacketLapData) -> anyhow::Result<TelemetryControl> {
//!         let frame = p.header.frame_identifier();
//!         println!("lap data frame {frame}");
//!         Ok(TelemetryControl::Continue)
//!     }
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> anyhow::Result<()> {
//!     TelemetryClient::new("0.0.0.0:20777").await?.listen(&mut MyHandler).await
//! }
//! ```
//!
//! ## Disclaimer
//! Not affiliated with Formula 1, the FIA, any F1 team, or EA/Codemasters.
//! Team names and identifiers are derived from the publicly available UDP specification
//! and are used solely for data interoperability.

pub mod endian;
pub mod enums;
pub mod macros;
pub mod packet_id;
pub mod packets;
pub mod parse;
pub mod wheel_data;

#[cfg(feature = "client")]
pub mod client;

pub use endian::FixEndianness;
pub use parse::parse;
pub use wheel_data::WheelData;

use std::mem::size_of;

use crate::constants::PACKET_HEADER_SIZE;
use macros::{wire_field_accessors, wire_index_accessors};

/// Wire-format packet header — the 29-byte prefix common to every packet.
///
/// All fields are primitive types; `packet_id` is a raw `u8` rather than an
/// enum so that no invalid-discriminant UB is possible when casting from bytes.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct PacketHeader {
    packet_format: u16,
    game_year: u8,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    /// Raw packet-id discriminant. See [`packet_id::PacketId`] for values.
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    overall_frame_identifier: u32,
    player_car_index: u8,
    /// `255` if no second player (split-screen).
    secondary_player_car_index: u8,
}

const _: () = assert!(size_of::<PacketHeader>() == PACKET_HEADER_SIZE);

/// A typed UDP packet: the 29-byte wire header followed by a packet-specific payload.
///
/// `T` is the wire-format payload struct. It must be `Copy` and `repr(C, packed)`
/// so that the overall `Packet<T>` has no hidden padding.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Packet<T> {
    pub header: PacketHeader,
    pub payload: T,
}

impl PacketHeader {
    /// Decode the raw packet-id byte into the typed [`packet_id::PacketId`] enum.
    ///
    /// Returns `Err(raw)` if the byte doesn't match any known variant.
    pub fn packet_id(self) -> Result<packet_id::PacketId, u8> {
        packet_id::PacketId::try_from(self.packet_id).map_err(|e| e.number)
    }

    wire_index_accessors!(player_car_index, secondary_player_car_index);

    wire_field_accessors!(
        packet_format: u16,
        game_year: u8,
        game_major_version: u8,
        game_minor_version: u8,
        packet_version: u8,
        session_uid: u64,
        session_time: f32,
        frame_identifier: u32,
        overall_frame_identifier: u32,
    );
}

impl FixEndianness for PacketHeader {
    fn fix_endianness(self) -> Self {
        Self {
            packet_format: self.packet_format.fix_endianness(),
            session_uid: self.session_uid.fix_endianness(),
            session_time: self.session_time.fix_endianness(),
            frame_identifier: self.frame_identifier.fix_endianness(),
            overall_frame_identifier: self.overall_frame_identifier.fix_endianness(),
            ..self
        }
    }
}

impl<T: Copy + FixEndianness> FixEndianness for Packet<T> {
    fn fix_endianness(self) -> Self {
        Self {
            header: self.header.fix_endianness(),
            payload: self.payload.fix_endianness(),
        }
    }
}

pub mod constants {
    pub const MAX_CARS_IN_SESSION: usize = 22;
    pub const MAX_TYRE_SETS: usize = 20; // 13 slick + 7 wet
    pub const PACKET_HEADER_SIZE: usize = 29;
    pub const PARTICIPANT_PACKET_SIZE: usize = 1284;
    pub const CAR_SETUP_PACKET_SIZE: usize = 1133;
    pub const CAR_STATUS_PACKET_SIZE: usize = 1239;
    pub const LAP_DATA_PACKET_SIZE: usize = 1285;
    pub const MOTION_DATA_PACKET_SIZE: usize = 1349;
    pub const EXTENDED_MOTION_DATA_PACKET_SIZE: usize = 273;
    pub const TELEMETRY_DATA_PACKET_SIZE: usize = 1352;
    pub const CLASSIFICATION_DATA_PACKET_SIZE: usize = 1042;
    pub const CAR_DAMAGE_DATA_PACKET_SIZE: usize = 1041;
    pub const SESSION_DATA_PACKET_SIZE: usize = 753;
    pub const SESSION_HISTORY_DATA_PACKET_SIZE: usize = 1460;
    pub const EVENT_DATA_PACKET_SIZE: usize = 45;
    pub const LOBBY_INFO_DATA_PACKET_SIZE: usize = 954;
    pub const TYRE_SETS_DATA_PACKET_SIZE: usize = 231;
    pub const TIME_TRIAL_DATA_PACKET_SIZE: usize = 101;
    pub const LAP_POSITIONS_DATA_PACKET_SIZE: usize = 1131;
}

/// Header information for F1 telemetry data packets.
///
/// This module defines the common header structure that is present in all F1 telemetry
/// data packets. The header contains metadata about the packet, such as its format,
/// the game version, session information, and more.
use serde::{
    Deserialize,
    Serialize,
};

/// Header structure for all F1 telemetry data packets.
///
/// Every telemetry packet begins with this header, which provides metadata about
/// the packet and the game session. This information can be used to identify the
/// packet type, the game version, and the session context.
///
/// # Fields
///
/// * `packet_format` - Format of the packet (game-specific)
/// * `game_year` - Game year (e.g., 23 for F1 2023)
/// * `game_major_version` - Major version of the game
/// * `game_minor_version` - Minor version of the game
/// * `packet_version` - Version of the packet format
/// * `packet_id` - Identifier for the packet type
/// * `session_uid` - Unique identifier for the session
/// * `session_time` - Session timestamp
/// * `frame_identifier` - Identifier for the frame
/// * `overall_frame_identifier` - Overall identifier for the frame
/// * `player_car_index` - Index of the player's car in the data arrays
/// * `secondary_player_car_index` - Index of the secondary player's car (split-screen)
#[derive(Deserialize, Debug, Default, Serialize, Copy, Clone, PartialEq)]
#[repr(packed)]
pub struct PacketHeader
{
    /// Format of the packet (game-specific)
    pub packet_format: u16,
    /// Game year (e.g., 23 for F1 2023)
    pub game_year: u8,
    /// Major version of the game
    pub game_major_version: u8,
    /// Minor version of the game
    pub game_minor_version: u8,
    /// Version of the packet format
    pub packet_version: u8,
    /// Identifier for the packet type
    pub packet_id: u8,
    /// Unique identifier for the session
    pub session_uid: u64,
    /// Session timestamp
    pub session_time: f32,
    /// Identifier for the frame
    pub frame_identifier: u32,
    /// Overall identifier for the frame
    pub overall_frame_identifier: u32,
    /// Index of the player's car in the data arrays
    #[serde(with = "crate::utils::u8_as_usize")]
    pub player_car_index: usize,
    /// Index of the secondary player's car (split-screen)
    #[serde(with = "crate::utils::u8_as_usize")]
    pub secondary_player_car_index: usize,
}

use serde::{Deserialize, Serialize};

use super::lobby_info_data::LobbyInfoData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

/// Packet containing lobby information for all players.
///
/// This structure contains information about all players in a multiplayer lobby.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `num_players` - Number of players in the lobby
/// * `lobby_players` - Array of lobby information for each player (up to 22 players)
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketLobbyInfoData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Number of players in the lobby
    pub num_players: u8,
    /// Array of lobby information for each player (up to 22 players)
    pub lobby_players: [LobbyInfoData; MAX_CARS_IN_SESSION],
}

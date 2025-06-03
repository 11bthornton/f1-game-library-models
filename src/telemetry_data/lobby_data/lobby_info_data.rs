use serde::{
    Deserialize,
    Serialize,
};
use serde_big_array::BigArray;

use super::{
    Platform,
    ReadyStatus,
};
use crate::telemetry_data::participant_data::{
    Nationality,
    Team,
};

/// Lobby information for a single player.
///
/// This structure contains detailed information about a single player
/// in a multiplayer lobby, including their team, nationality, and readiness status.
///
/// # Fields
///
/// * `ai_controlled` - Whether the player is AI or human controlled
/// * `team` - Team selected by the player
/// * `nationality` - Nationality of the player
/// * `platform` - Gaming platform the player is using
/// * `name` - Name of the player
/// * `car_number` - Car number selected by the player
/// * `ready_status` - Readiness status of the player
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct LobbyInfoData
{
    /// Whether the player is AI (true) or Human (false) controlled
    pub ai_controlled: bool,
    /// Team selected by the player (255 if no team currently selected)
    pub team: Team,
    /// Nationality of the player
    pub nationality: Nationality,
    /// Gaming platform the player is using (Steam, PlayStation, Xbox, Origin, etc.)
    pub platform: Platform,

    /// Name of player in UTF-8 format (null terminated, truncated with ... if too long)
    #[serde(with = "BigArray")]
    pub name: [u8; 32],
    /// Car number selected by the player
    pub car_number: u8,

    pub your_telemetry: bool,

    pub show_online_names: bool,

    pub tech_level: u16,

    /// Readiness status of the player (not ready, ready, spectating)
    pub ready_status: ReadyStatus,
}

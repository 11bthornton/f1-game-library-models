/// Lobby information data from F1 games.
///
/// This module contains structures for representing multiplayer lobby
/// information in F1 games, including player details, teams, and readiness status.
mod lobby_info_data;
mod packet;
mod platform;
mod ready_status;

// Re-export the structs and enums
pub use lobby_info_data::LobbyInfoData;
pub use packet::PacketLobbyInfoData;
pub use platform::Platform;
pub use ready_status::ReadyStatus;

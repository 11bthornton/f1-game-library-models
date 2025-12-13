use serde::Serialize;
use serde_repr::Deserialize_repr;

/// Player readiness status.
///
/// These represent the various readiness states a player can have in a lobby.
#[derive(Debug, Deserialize_repr, Default, Serialize, Clone, Copy)]
#[repr(u8)]
pub enum ReadyStatus {
    /// Player is not ready to start
    #[serde(rename = "Not Ready")]
    #[default]
    NotReady,
    /// Player is ready to start
    Ready,
    /// Player is spectating the session
    Spectating,
}

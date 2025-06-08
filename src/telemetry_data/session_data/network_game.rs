//! Defines the network game status.
//! 
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Network game status
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum NetworkGame {
    /// Offline game
    Offline,
    /// Online multiplayer game
    Online,
}

impl NetworkGame {
    pub fn is_online(&self) -> bool {
        matches!(self, Self::Online)
    }

    pub fn is_offline(&self) -> bool {
        matches!(self, Self::Offline)
    }
}

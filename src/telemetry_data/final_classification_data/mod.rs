//! Final classification data from F1 games.
//!
//! This module contains structures for representing final race classification
//! information in F1 games, including finishing positions, lap times, and tyre usage.
mod final_classification_data;
mod packet;

// Re-export the structs
pub use final_classification_data::FinalClassificationData;
pub use packet::PacketClassificationData;

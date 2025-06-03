/// Extended motion data from F1 games.
///
/// This module contains structures for representing extended physics and motion
/// information in F1 games, focusing on detailed wheel, suspension, and vehicle dynamics.
mod packet;

// Re-export the structs
pub use packet::PacketMotionExData;

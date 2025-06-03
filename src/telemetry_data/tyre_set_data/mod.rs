/// Tyre set data from F1 games.
///
/// This module contains structures for representing tyre set information
/// in F1 games, including compound types, wear, and availability.
mod packet;
mod tyre_set;

// Re-export the structs
pub use packet::*;
pub use tyre_set::*;

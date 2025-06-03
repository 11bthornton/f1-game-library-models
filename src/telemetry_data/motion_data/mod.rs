/// Motion data from F1 games.
///
/// This module contains structures for representing car motion and physics
/// information in F1 games, including position, velocity, and orientation data.
mod car_motion_data;
mod packet;

// Re-export the structs
pub use car_motion_data::CarMotionData;
pub use packet::PacketMotionData;

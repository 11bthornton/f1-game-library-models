//! Defines the `CarMotionData` struct which contains motion data for a single vehicle.

use serde::{Deserialize, Serialize};

/// Car motion data for a single vehicle.
///
/// Contains physics data for a single car such as position, rotation, velocity, and acceleration.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CarMotionData {
    /// World space X position - meters
    pub world_position_x: f32,
    /// World space Y position - meters
    pub world_position_y: f32,
    /// World space Z position - meters
    pub world_position_z: f32,
    /// Velocity in world space X - meters per second
    pub world_velocity_x: f32,
    /// Velocity in world space Y - meters per second
    pub world_velocity_y: f32,
    /// Velocity in world space Z - meters per second
    pub world_velocity_z: f32,
    /// World space forward X direction (normalized)
    pub world_forward_dir_x: i16,
    /// World space forward Y direction (normalized)
    pub world_forward_dir_y: i16,
    /// World space forward Z direction (normalized)
    pub world_forward_dir_z: i16,
    /// World space right X direction (normalized)
    pub world_right_dir_x: i16,
    /// World space right Y direction (normalized)
    pub world_right_dir_y: i16,
    /// World space right Z direction (normalized)
    pub world_right_dir_z: i16,
    /// Lateral G-Force component
    pub g_force_lateral: f32,
    /// Longitudinal G-Force component
    pub g_force_longitudinal: f32,
    /// Vertical G-Force component
    pub g_force_vertical: f32,
    /// Yaw angle in radians
    pub yaw: f32,
    /// Pitch angle in radians
    pub pitch: f32,
    /// Roll angle in radians
    pub roll: f32,
}

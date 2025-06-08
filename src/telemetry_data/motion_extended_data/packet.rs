//! Defines the extended motion data packet structure for the player's car in F1 telemetry.

use serde::{Deserialize, Serialize};

use crate::telemetry_data::{WheelData, packet_header::PacketHeader};

/// Extended motion data packet for the player's car.
///
/// This structure contains detailed physics information specifically for the player's car,
/// including suspension, wheel forces, and vehicle dynamics that are not available
/// in the standard motion data packet.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `suspension_position` - Suspension position for each wheel
/// * `suspension_velocity` - Suspension velocity for each wheel
/// * `suspension_acceleration` - Suspension acceleration for each wheel
/// * `wheel_speed` - Speed of each wheel
/// * `wheel_slip_ratio` - Slip ratio for each wheel
/// * `wheel_slip_angle` - Slip angles for each wheel
/// * `wheel_lat_force` - Lateral forces for each wheel
/// * `wheel_long_force` - Longitudinal forces for each wheel
/// * `height_of_cog_above_ground` - Height of center of gravity above ground
/// * `local_velocity_x` - Velocity in local space X
/// * `local_velocity_y` - Velocity in local space Y
/// * `local_velocity_z` - Velocity in local space Z
/// * `angular_velocity_x` - Angular velocity x-component
/// * `angular_velocity_y` - Angular velocity y-component
/// * `angular_velocity_z` - Angular velocity z-component
/// * `angular_acceleration_x` - Angular acceleration x-component
/// * `angular_acceleration_y` - Angular acceleration y-component
/// * `angular_acceleration_z` - Angular acceleration z-component
/// * `front_wheels_angle` - Current front wheels angle
/// * `wheel_vert_force` - Vertical forces for each wheel
///
/// Note: All wheel arrays have the following order: RL (Rear Left), RR (Rear Right), FL (Front
/// Left), FR (Front Right)
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PacketMotionExData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Suspension position for each wheel [RL, RR, FL, FR]
    pub suspension_position: WheelData<f32>,
    /// Suspension velocity for each wheel [RL, RR, FL, FR]
    pub suspension_velocity: WheelData<f32>,
    /// Suspension acceleration for each wheel [RL, RR, FL, FR]
    pub suspension_acceleration: WheelData<f32>,
    /// Speed of each wheel [RL, RR, FL, FR]
    pub wheel_speed: WheelData<f32>,
    /// Slip ratio for each wheel [RL, RR, FL, FR]
    pub wheel_slip_ratio: WheelData<f32>,
    /// Slip angles for each wheel [RL, RR, FL, FR]
    pub wheel_slip_angle: WheelData<f32>,
    /// Lateral forces for each wheel [RL, RR, FL, FR]
    pub wheel_lat_force: WheelData<f32>,
    /// Longitudinal forces for each wheel [RL, RR, FL, FR]
    pub wheel_long_force: WheelData<f32>,
    /// Height of center of gravity above ground in meters
    pub height_of_cog_above_ground: f32,
    /// Velocity in local space X in meters/s
    pub local_velocity_x: f32,
    /// Velocity in local space Y in meters/s
    pub local_velocity_y: f32,
    /// Velocity in local space Z in meters/s
    pub local_velocity_z: f32,
    /// Angular velocity x-component in radians/s
    pub angular_velocity_x: f32,
    /// Angular velocity y-component in radians/s
    pub angular_velocity_y: f32,
    /// Angular velocity z-component in radians/s
    pub angular_velocity_z: f32,
    /// Angular acceleration x-component in radians/s²
    pub angular_acceleration_x: f32,
    /// Angular acceleration y-component in radians/s²
    pub angular_acceleration_y: f32,
    /// Angular acceleration z-component in radians/s²
    pub angular_acceleration_z: f32,
    /// Current front wheels angle in radians
    pub front_wheels_angle: f32,
    /// Vertical forces for each wheel [RL, RR, FL, FR]
    pub wheel_vert_force: WheelData<f32>,
    /// Front aero height
    pub front_aero_height: f32,
    /// Rear aero height
    pub rear_aero_height: f32,
    /// Front roll angle in radians
    pub front_roll_angle: f32,
    /// Rear roll angle in radians
    pub rear_roll_angle: f32,
    /// Chassis yaw in radians
    pub chassis_yaw: f32,
    /// Chassis roll in radians
    pub chassis_pitch: f32,
    /// Chassis pitch in radians
    pub wheel_camber: WheelData<f32>,
    /// Wheel camber gain for each wheel [RL, RR, FL, FR]
    pub wheel_camber_gain: WheelData<f32>,
}

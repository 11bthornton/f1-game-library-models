use serde::{Deserialize, Serialize};

use super::CarMotionData;
use crate::telemetry_data::packet_header::PacketHeader;

/// Motion data packet containing car physics information.
///
/// This packet details car motion and physics data for all cars in the race.
/// It includes arrays of motion data for each car, as well as additional data
/// for the player's car.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PacketMotionData {
    pub packet_header: PacketHeader,

    /// Motion data for all cars on track
    pub car_motion_data: [CarMotionData; 22],
    // /// Suspension position of each wheel [RL, RR, FL, FR] - meters
    // pub suspension_position: WheelData<f32>,
    // /// Suspension velocity of each wheel [RL, RR, FL, FR] - meters/second
    // pub suspension_velocity: WheelData<f32>,
    // /// Suspension acceleration of each wheel [RL, RR, FL, FR] - meters/second/second
    // pub suspension_acceleration: WheelData<f32>,
    // /// Wheel speed of each wheel [RL, RR, FL, FR] - meters/second
    // pub wheel_speed: WheelData<f32>,
    // /// Slip ratio of each wheel [RL, RR, FL, FR] - 0-100%
    // pub wheel_slip: WheelData<f32>,
    // /// Velocity in local space X - meters/second
    // pub local_velocity_x: f32,
    // /// Velocity in local space Y - meters/second
    // pub local_velocity_y: f32,
    // /// Velocity in local space Z - meters/second
    // pub local_velocity_z: f32,
    // /// Angular velocity X - radians/second
    // pub angular_velocity_x: f32,
    // /// Angular velocity Y - radians/second
    // pub angular_velocity_y: f32,
    // /// Angular velocity Z - radians/second
    // pub angular_velocity_z: f32,
    // /// Angular acceleration X - radians/second/second
    // pub angular_acceleration_x: f32,
    // /// Angular acceleration Y - radians/second/second
    // pub angular_acceleration_y: f32,
    // /// Angular acceleration Z - radians/second/second
    // pub angular_acceleration_z: f32,
    // /// Current front wheels angle in radians
    // pub front_wheels_angle: f32,
}

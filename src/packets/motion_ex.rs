//! Wire-format structures for the Motion Ex packet (packet id 13).
//!
//! Spec: `PacketMotionExData` — 273 bytes total.
//! Data struct: `MotionExData` — 244 bytes.
//!
//! Player car only. All fields are `f32` or `WheelData<f32>`.
//! Wheel order throughout: RL, RR, FL, FR.

use std::mem::size_of;

use crate::constants::{EXTENDED_MOTION_DATA_PACKET_SIZE, PACKET_HEADER_SIZE};

use super::super::{Packet, WheelData, endian::FixEndianness};

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MotionExData {
    pub suspension_position: WheelData<f32>,
    pub suspension_velocity: WheelData<f32>,
    pub suspension_acceleration: WheelData<f32>,
    pub wheel_speed: WheelData<f32>,
    pub wheel_slip_ratio: WheelData<f32>,
    pub wheel_slip_angle: WheelData<f32>,
    pub wheel_lat_force: WheelData<f32>,
    pub wheel_long_force: WheelData<f32>,
    pub height_of_cog_above_ground: f32,
    pub local_velocity_x: f32,
    pub local_velocity_y: f32,
    pub local_velocity_z: f32,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub angular_acceleration_x: f32,
    pub angular_acceleration_y: f32,
    pub angular_acceleration_z: f32,
    pub front_wheels_angle: f32,
    pub wheel_vert_force: WheelData<f32>,
    pub front_aero_height: f32,
    pub rear_aero_height: f32,
    pub front_roll_angle: f32,
    pub rear_roll_angle: f32,
    pub chassis_yaw: f32,
    pub chassis_pitch: f32,
    pub wheel_camber: WheelData<f32>,
    pub wheel_camber_gain: WheelData<f32>,
}

const _: () = assert!(size_of::<MotionExData>() == EXTENDED_MOTION_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format motion ex packet (player car only).
///
/// `header.packet_id` will be `13` for this packet type.
pub type PacketMotionEx = Packet<MotionExData>;

const _: () = assert!(size_of::<PacketMotionEx>() == EXTENDED_MOTION_DATA_PACKET_SIZE);

impl FixEndianness for MotionExData {
    fn fix_endianness(self) -> Self {
        Self {
            suspension_position: self.suspension_position.fix_endianness(),
            suspension_velocity: self.suspension_velocity.fix_endianness(),
            suspension_acceleration: self.suspension_acceleration.fix_endianness(),
            wheel_speed: self.wheel_speed.fix_endianness(),
            wheel_slip_ratio: self.wheel_slip_ratio.fix_endianness(),
            wheel_slip_angle: self.wheel_slip_angle.fix_endianness(),
            wheel_lat_force: self.wheel_lat_force.fix_endianness(),
            wheel_long_force: self.wheel_long_force.fix_endianness(),
            height_of_cog_above_ground: self.height_of_cog_above_ground.fix_endianness(),
            local_velocity_x: self.local_velocity_x.fix_endianness(),
            local_velocity_y: self.local_velocity_y.fix_endianness(),
            local_velocity_z: self.local_velocity_z.fix_endianness(),
            angular_velocity_x: self.angular_velocity_x.fix_endianness(),
            angular_velocity_y: self.angular_velocity_y.fix_endianness(),
            angular_velocity_z: self.angular_velocity_z.fix_endianness(),
            angular_acceleration_x: self.angular_acceleration_x.fix_endianness(),
            angular_acceleration_y: self.angular_acceleration_y.fix_endianness(),
            angular_acceleration_z: self.angular_acceleration_z.fix_endianness(),
            front_wheels_angle: self.front_wheels_angle.fix_endianness(),
            wheel_vert_force: self.wheel_vert_force.fix_endianness(),
            front_aero_height: self.front_aero_height.fix_endianness(),
            rear_aero_height: self.rear_aero_height.fix_endianness(),
            front_roll_angle: self.front_roll_angle.fix_endianness(),
            rear_roll_angle: self.rear_roll_angle.fix_endianness(),
            chassis_yaw: self.chassis_yaw.fix_endianness(),
            chassis_pitch: self.chassis_pitch.fix_endianness(),
            wheel_camber: self.wheel_camber.fix_endianness(),
            wheel_camber_gain: self.wheel_camber_gain.fix_endianness(),
        }
    }
}

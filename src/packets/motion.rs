//! Wire-format structures for the Motion packet (packet id 0).
//!
//! Spec: `PacketMotionData` — 1349 bytes total.
//! Per-car struct: `CarMotionData` — 60 bytes.

use std::mem::size_of;

use crate::constants::{MAX_CARS_IN_SESSION, MOTION_DATA_PACKET_SIZE, PACKET_HEADER_SIZE};

use super::super::{Packet, endian::FixEndianness, macros::wire_field_accessors};

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarMotionData {
    world_position_x: f32,
    world_position_y: f32,
    world_position_z: f32,
    world_velocity_x: f32,
    world_velocity_y: f32,
    world_velocity_z: f32,
    /// World space forward direction (normalised), X component.
    world_forward_dir_x: i16,
    world_forward_dir_y: i16,
    world_forward_dir_z: i16,
    /// World space right direction (normalised), X component.
    world_right_dir_x: i16,
    world_right_dir_y: i16,
    world_right_dir_z: i16,
    g_force_lateral: f32,
    g_force_longitudinal: f32,
    g_force_vertical: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl CarMotionData {
    wire_field_accessors!(
        world_position_x: f32,
        world_position_y: f32,
        world_position_z: f32,
        world_velocity_x: f32,
        world_velocity_y: f32,
        world_velocity_z: f32,
        world_forward_dir_x: i16,
        world_forward_dir_y: i16,
        world_forward_dir_z: i16,
        world_right_dir_x: i16,
        world_right_dir_y: i16,
        world_right_dir_z: i16,
        g_force_lateral: f32,
        g_force_longitudinal: f32,
        g_force_vertical: f32,
        yaw: f32,
        pitch: f32,
        roll: f32,
    );
}

const _: () =
    assert!(size_of::<CarMotionData>() == (MOTION_DATA_PACKET_SIZE - PACKET_HEADER_SIZE) / MAX_CARS_IN_SESSION);

/// Wire-format motion packet for all cars in the session.
///
/// `header.packet_id` will be `0` for this packet type.
pub type PacketMotion = Packet<[CarMotionData; MAX_CARS_IN_SESSION]>;

const _: () = assert!(size_of::<PacketMotion>() == MOTION_DATA_PACKET_SIZE);

impl FixEndianness for CarMotionData {
    fn fix_endianness(self) -> Self {
        Self {
            world_position_x: self.world_position_x.fix_endianness(),
            world_position_y: self.world_position_y.fix_endianness(),
            world_position_z: self.world_position_z.fix_endianness(),
            world_velocity_x: self.world_velocity_x.fix_endianness(),
            world_velocity_y: self.world_velocity_y.fix_endianness(),
            world_velocity_z: self.world_velocity_z.fix_endianness(),
            world_forward_dir_x: self.world_forward_dir_x.fix_endianness(),
            world_forward_dir_y: self.world_forward_dir_y.fix_endianness(),
            world_forward_dir_z: self.world_forward_dir_z.fix_endianness(),
            world_right_dir_x: self.world_right_dir_x.fix_endianness(),
            world_right_dir_y: self.world_right_dir_y.fix_endianness(),
            world_right_dir_z: self.world_right_dir_z.fix_endianness(),
            g_force_lateral: self.g_force_lateral.fix_endianness(),
            g_force_longitudinal: self.g_force_longitudinal.fix_endianness(),
            g_force_vertical: self.g_force_vertical.fix_endianness(),
            yaw: self.yaw.fix_endianness(),
            pitch: self.pitch.fix_endianness(),
            roll: self.roll.fix_endianness(),
        }
    }
}

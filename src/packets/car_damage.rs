//! Wire-format structures for the Car Damage packet (packet id 10).
//!
//! Spec: `PacketCarDamageData` — 1041 bytes total.
//! Per-car struct: `CarDamageData` — 46 bytes.
//!
//! All fault/fault-flag fields (`drs_fault`, `ers_fault`, `engine_blown`,
//! `engine_seized`) are raw `u8` on the wire: `0` = OK, `1` = fault.

use std::mem::size_of;

use crate::constants::{CAR_DAMAGE_DATA_PACKET_SIZE, MAX_CARS_IN_SESSION};

use super::super::{Packet, WheelData, endian::FixEndianness, macros::wire_flag_accessors};

/// Wire-format damage data for a single car.
///
/// Field layout mirrors the official C++ struct exactly:
///
/// ```text
/// float    m_tyresWear[4]
/// uint8    m_tyresDamage[4]
/// uint8    m_brakesDamage[4]
/// uint8    m_tyreBlisters[4]
/// uint8    m_frontLeftWingDamage
/// uint8    m_frontRightWingDamage
/// uint8    m_rearWingDamage
/// uint8    m_floorDamage
/// uint8    m_diffuserDamage
/// uint8    m_sidepodDamage
/// uint8    m_drsFault           // 0 = OK, 1 = fault
/// uint8    m_ersFault           // 0 = OK, 1 = fault
/// uint8    m_gearBoxDamage
/// uint8    m_engineDamage
/// uint8    m_engineMGUHWear
/// uint8    m_engineESWear
/// uint8    m_engineCEWear
/// uint8    m_engineICEWear
/// uint8    m_engineMGUKWear
/// uint8    m_engineTCWear
/// uint8    m_engineBlown        // 0 = OK, 1 = fault
/// uint8    m_engineSeized       // 0 = OK, 1 = fault
/// ```
///
/// Wheel order for array fields: `[rear_left, rear_right, front_left, front_right]`.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarDamageData {
    /// Tyre wear percentage per wheel.
    pub tyre_wear: WheelData<f32>,
    /// Tyre damage percentage per wheel.
    pub tyre_damage: WheelData<u8>,
    /// Brake damage percentage per wheel.
    pub brakes_damage: WheelData<u8>,
    /// Tyre blisters percentage per wheel.
    pub tyre_blisters: WheelData<u8>,
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub floor_damage: u8,
    pub diffuser_damage: u8,
    pub sidepod_damage: u8,
    /// DRS fault indicator: `0` = OK, `1` = fault.
    drs_fault: u8,
    /// ERS fault indicator: `0` = OK, `1` = fault.
    ers_fault: u8,
    pub gear_box_damage: u8,
    pub engine_damage: u8,
    pub engine_mguh_wear: u8,
    pub engine_es_wear: u8,
    pub engine_ce_wear: u8,
    pub engine_ice_wear: u8,
    pub engine_mguk_wear: u8,
    pub engine_tc_wear: u8,
    /// Engine blown indicator: `0` = OK, `1` = fault.
    engine_blown: u8,
    /// Engine seized indicator: `0` = OK, `1` = fault.
    engine_seized: u8,
}

impl CarDamageData {
    wire_flag_accessors!(drs_fault, ers_fault, engine_blown, engine_seized);
}

const _: () = assert!(
    size_of::<CarDamageData>()
        == (CAR_DAMAGE_DATA_PACKET_SIZE - crate::constants::PACKET_HEADER_SIZE) / MAX_CARS_IN_SESSION
);

/// Wire-format car damage packet for all cars in the session.
///
/// `header.packet_id` will be `10` for this packet type.
pub type PacketCarDamage = Packet<[CarDamageData; MAX_CARS_IN_SESSION]>;

const _: () = assert!(size_of::<PacketCarDamage>() == CAR_DAMAGE_DATA_PACKET_SIZE);

impl FixEndianness for CarDamageData {
    fn fix_endianness(self) -> Self {
        Self {
            tyre_wear: self.tyre_wear.fix_endianness(),
            // all remaining fields are u8 — no byte-swapping needed
            ..self
        }
    }
}

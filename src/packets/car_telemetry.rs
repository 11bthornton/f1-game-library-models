//! Wire-format structures for the Car Telemetry packet (packet id 6).
//!
//! Spec: `PacketCarTelemetryData` — 1352 bytes total.
//! Per-car struct: `CarTelemetryData` — 60 bytes.

use std::mem::size_of;

use crate::constants::{MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE, TELEMETRY_DATA_PACKET_SIZE};

use super::super::{Packet, WheelData, endian::FixEndianness, enums::SurfaceType, macros::{wire_field_accessors, wire_flag_accessors}};

/// Wire-format telemetry data for a single car.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarTelemetryData {
    /// Speed of car in kilometres per hour.
    speed: u16,
    /// Throttle application (0.0 to 1.0).
    throttle: f32,
    /// Steering (-1.0 full lock left to 1.0 full lock right).
    steer: f32,
    /// Brake application (0.0 to 1.0).
    brake: f32,
    /// Clutch application (0 to 100).
    pub clutch: u8,
    /// Gear selected (1–8, N=0, R=−1).
    pub gear: i8,
    /// Engine RPM.
    engine_rpm: u16,
    drs: u8,
    /// Rev lights indicator (percentage).
    pub rev_lights_percent: u8,
    /// Rev lights as a bitfield (bit 0 = leftmost LED, bit 14 = rightmost LED).
    rev_lights_bit_value: u16,
    /// Brake temperatures per wheel (Celsius). Wheel order: RL, RR, FL, FR.
    brakes_temperature: WheelData<u16>,
    /// Tyre surface temperatures per wheel (Celsius).
    pub tyres_surface_temperature: WheelData<u8>,
    /// Tyre inner temperatures per wheel (Celsius).
    pub tyres_inner_temperature: WheelData<u8>,
    /// Engine temperature (Celsius).
    engine_temperature: u16,
    /// Tyre pressures per wheel (PSI).
    tyres_pressure: WheelData<f32>,
    surface_type_raw: WheelData<u8>,
}

const _: () = assert!(size_of::<CarTelemetryData>() == 60);

impl CarTelemetryData {
    wire_flag_accessors!(drs);

    wire_field_accessors!(
        speed: u16,
        throttle: f32,
        steer: f32,
        brake: f32,
        engine_rpm: u16,
        rev_lights_bit_value: u16,
        brakes_temperature: WheelData<u16>,
        engine_temperature: u16,
        tyres_pressure: WheelData<f32>,
    );

    /// Driving surface under each wheel. `Err(raw)` if the value is unrecognised.
    /// Wheel order: RL, RR, FL, FR.
    pub fn surface_type(self) -> WheelData<Result<SurfaceType, u8>> {
        let convert = |v: u8| SurfaceType::try_from(v).map_err(|_| v);
        WheelData {
            rear_left: convert(self.surface_type_raw.rear_left),
            rear_right: convert(self.surface_type_raw.rear_right),
            front_left: convert(self.surface_type_raw.front_left),
            front_right: convert(self.surface_type_raw.front_right),
        }
    }
}

impl FixEndianness for CarTelemetryData {
    fn fix_endianness(self) -> Self {
        Self {
            speed: self.speed.fix_endianness(),
            throttle: self.throttle.fix_endianness(),
            steer: self.steer.fix_endianness(),
            brake: self.brake.fix_endianness(),
            engine_rpm: self.engine_rpm.fix_endianness(),
            rev_lights_bit_value: self.rev_lights_bit_value.fix_endianness(),
            brakes_temperature: self.brakes_temperature.fix_endianness(),
            engine_temperature: self.engine_temperature.fix_endianness(),
            tyres_pressure: self.tyres_pressure.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format car telemetry payload for all cars plus MFD fields.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CarTelemetryPayload {
    pub car_telemetry_data: [CarTelemetryData; MAX_CARS_IN_SESSION],
    /// Index of the MFD panel open (255 = closed).
    pub mfd_panel_index: u8,
    /// MFD panel index for the secondary player (split-screen).
    pub mfd_panel_index_secondary_player: u8,
    /// Suggested gear for the player (1–8), 0 if none.
    pub suggested_gear: i8,
}

const _: () = assert!(size_of::<CarTelemetryPayload>() == TELEMETRY_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format car telemetry packet for all cars in the session.
///
/// `header.packet_id` will be `6` for this packet type.
pub type PacketCarTelemetry = Packet<CarTelemetryPayload>;

const _: () = assert!(size_of::<PacketCarTelemetry>() == TELEMETRY_DATA_PACKET_SIZE);

impl FixEndianness for CarTelemetryPayload {
    fn fix_endianness(self) -> Self {
        Self {
            car_telemetry_data: self.car_telemetry_data.fix_endianness(),
            ..self
        }
    }
}

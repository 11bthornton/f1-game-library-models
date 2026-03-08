//! Wire-format structures for the Session packet (packet id 1).
//!
//! Spec: `PacketSessionData` — 753 bytes total.
//! Single struct with nested arrays (21 marshal zones, 64 weather forecast samples).

pub mod enums;

use std::mem::size_of;

use crate::constants::{PACKET_HEADER_SIZE, SESSION_DATA_PACKET_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::SafetyCarType,
    macros::{wire_enum_accessors, wire_flag_accessors, wire_index_accessors},
};
use enums::*;

const MAX_MARSHAL_ZONES: usize = 21;
const MAX_WEATHER_FORECAST_SAMPLES: usize = 64;
const MAX_SESSIONS_IN_WEEKEND: usize = 12;

/// A single marshal zone on the circuit.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MarshalZone {
    /// Fraction (0.0–1.0) of the way through the lap where this zone starts.
    pub zone_start: f32,
    /// Flag shown in this zone: -1 = invalid, 0 = none, 1 = green, 2 = blue, 3 = yellow.
    pub zone_flag: i8,
}

const _: () = assert!(size_of::<MarshalZone>() == 5);

impl FixEndianness for MarshalZone {
    fn fix_endianness(self) -> Self {
        Self {
            zone_start: self.zone_start.fix_endianness(),
            ..self
        }
    }
}

/// A single weather forecast sample.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct WeatherForecastSample {
    /// Session type this forecast applies to (see appendix).
    pub session_type: u8,
    /// Minutes into the future this forecast is for.
    pub time_offset: u8,
    weather: u8,
    /// Forecast track temperature (Celsius).
    pub track_temperature: i8,
    /// Track temperature trend: 0 = up, 1 = down, 2 = no change.
    pub track_temperature_change: i8,
    /// Forecast air temperature (Celsius).
    pub air_temperature: i8,
    /// Air temperature trend: 0 = up, 1 = down, 2 = no change.
    pub air_temperature_change: i8,
    /// Rain probability (0–100).
    pub rain_percentage: u8,
}

const _: () = assert!(size_of::<WeatherForecastSample>() == 8);

impl WeatherForecastSample {
    wire_enum_accessors!(weather => Weather);
}

impl FixEndianness for WeatherForecastSample {
    fn fix_endianness(self) -> Self {
        // All fields are u8 or i8 — no byte swapping needed.
        self
    }
}

/// Wire-format session data payload.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SessionData {
    weather: u8,
    /// Track temperature (Celsius).
    pub track_temperature: i8,
    /// Air temperature (Celsius).
    pub air_temperature: i8,
    /// Total number of laps in the race.
    pub total_laps: u8,
    /// Track length in metres.
    pub track_length: u16,
    /// Session type (see appendix). `0` = unknown.
    pub session_type: u8,
    /// Track id (see appendix). `-1` = unknown.
    pub track_id: i8,
    formula: u8,
    /// Time remaining in the session (seconds).
    pub session_time_left: u16,
    /// Total session duration (seconds).
    pub session_duration: u16,
    /// Pit lane speed limit (km/h).
    pub pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    /// Index of the car being spectated.
    spectator_car_index: u8,
    sli_pro_native_support: u8,
    /// Number of valid marshal zones in `marshal_zones`.
    pub num_marshal_zones: u8,
    pub marshal_zones: [MarshalZone; MAX_MARSHAL_ZONES],
    safety_car_status: u8,
    network_game: u8,
    /// Number of valid entries in `weather_forecast_samples`.
    pub num_weather_forecast_samples: u8,
    pub weather_forecast_samples: [WeatherForecastSample; MAX_WEATHER_FORECAST_SAMPLES],
    forecast_accuracy: u8,
    /// AI difficulty (0–110).
    pub ai_difficulty: u8,
    /// Season link identifier (persists across saves).
    pub season_link_identifier: u32,
    /// Weekend link identifier (persists across saves).
    pub weekend_link_identifier: u32,
    /// Session link identifier (persists across saves).
    pub session_link_identifier: u32,
    /// Ideal lap to pit on for current strategy (player).
    pub pit_stop_window_ideal_lap: u8,
    /// Latest lap to pit on for current strategy (player).
    pub pit_stop_window_latest_lap: u8,
    /// Predicted rejoin position after pit (player).
    pub pit_stop_rejoin_position: u8,
    steering_assist: u8,
    braking_assist: u8,
    gearbox_assist: u8,
    pit_assist: u8,
    pit_release_assist: u8,
    ers_assist: u8,
    drs_assist: u8,
    dynamic_racing_line: u8,
    dynamic_racing_line_type: u8,
    /// Game mode id (see appendix).
    pub game_mode: u8,
    /// Ruleset id (see appendix).
    pub rule_set: u8,
    /// Local time of day in minutes since midnight.
    pub time_of_day: u32,
    session_length: u8,
    speed_units_lead_player: u8,
    temperature_units_lead_player: u8,
    speed_units_secondary_player: u8,
    temperature_units_secondary_player: u8,
    /// Number of safety car periods in the session.
    pub num_safety_car_periods: u8,
    /// Number of virtual safety car periods in the session.
    pub num_virtual_safety_car_periods: u8,
    /// Number of red flag periods in the session.
    pub num_red_flag_periods: u8,
    equal_car_performance: u8,
    recovery_mode: u8,
    flashback_limit: u8,
    /// Surface simulation setting (0 = Simplified, 1 = Realistic).
    pub surface_type: u8,
    low_fuel_mode: u8,
    race_starts: u8,
    tyre_temperature: u8,
    pit_lane_tyre_sim: u8,
    car_damage: u8,
    car_damage_rate: u8,
    collisions: u8,
    collisions_off_for_first_lap_only: u8,
    mp_unsafe_pit_release: u8,
    mp_off_for_griefing: u8,
    corner_cutting_stringency: u8,
    parc_ferme_rules: u8,
    pit_stop_experience: u8,
    safety_car: u8,
    safety_car_experience: u8,
    formation_lap: u8,
    formation_lap_experience: u8,
    red_flags: u8,
    affects_licence_level_solo: u8,
    affects_licence_level_mp: u8,
    /// Number of valid entries in `weekend_structure`.
    pub num_sessions_in_weekend: u8,
    /// Weekend session schedule (see appendix for session type values).
    pub weekend_structure: [u8; MAX_SESSIONS_IN_WEEKEND],
    /// Distance around the lap where sector 2 starts (metres).
    pub sector2_lap_distance_start: f32,
    /// Distance around the lap where sector 3 starts (metres).
    pub sector3_lap_distance_start: f32,
}

const _: () = assert!(size_of::<SessionData>() == SESSION_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

impl SessionData {
    wire_index_accessors!(spectator_car_index);

    wire_flag_accessors!(
        game_paused,
        is_spectating,
        sli_pro_native_support,
        network_game,
        steering_assist,
        pit_assist,
        pit_release_assist,
        ers_assist,
        drs_assist,
        equal_car_performance,
        low_fuel_mode,
        race_starts,
        tyre_temperature,
        pit_lane_tyre_sim,
        formation_lap,
        formation_lap_experience,
        safety_car_experience,
        parc_ferme_rules,
        collisions_off_for_first_lap_only,
        mp_unsafe_pit_release,
        mp_off_for_griefing,
        corner_cutting_stringency,
        affects_licence_level_solo,
        affects_licence_level_mp,
    );

    wire_enum_accessors!(
        weather                            => Weather,
        formula                            => Formula,
        safety_car_status                  => SafetyCarType,
        forecast_accuracy                  => ForecastAccuracy,
        braking_assist                     => BrakingAssist,
        gearbox_assist                     => GearboxAssist,
        dynamic_racing_line                => DynamicRacingLine,
        dynamic_racing_line_type           => DynamicRacingLineType,
        session_length                     => SessionLength,
        speed_units_lead_player            => SpeedUnits,
        temperature_units_lead_player      => TemperatureUnits,
        speed_units_secondary_player       => SpeedUnits,
        temperature_units_secondary_player => TemperatureUnits,
        recovery_mode                      => RecoveryMode,
        flashback_limit                    => FlashbackLimit,
        car_damage                         => CarDamageLevel,
        car_damage_rate                    => CarDamageRate,
        collisions                         => CollisionSetting,
        pit_stop_experience                => PitStopExperience,
        safety_car                         => IncidentFrequency,
        red_flags                          => IncidentFrequency,
    );
}

impl FixEndianness for SessionData {
    fn fix_endianness(self) -> Self {
        Self {
            track_length: self.track_length.fix_endianness(),
            session_time_left: self.session_time_left.fix_endianness(),
            session_duration: self.session_duration.fix_endianness(),
            marshal_zones: self.marshal_zones.fix_endianness(),
            season_link_identifier: self.season_link_identifier.fix_endianness(),
            weekend_link_identifier: self.weekend_link_identifier.fix_endianness(),
            session_link_identifier: self.session_link_identifier.fix_endianness(),
            time_of_day: self.time_of_day.fix_endianness(),
            sector2_lap_distance_start: self.sector2_lap_distance_start.fix_endianness(),
            sector3_lap_distance_start: self.sector3_lap_distance_start.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format session packet.
///
/// `header.packet_id` will be `1` for this packet type.
pub type PacketSession = Packet<SessionData>;

const _: () = assert!(size_of::<PacketSession>() == SESSION_DATA_PACKET_SIZE);

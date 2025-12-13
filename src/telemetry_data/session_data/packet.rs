//! Defines the session data packet structure.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{
    braking_assist::BrakingAssist, car_damage::CarDamage, car_damage_rate::CarDamageRate,
    collisions::Collisions, corner_cutting_stringency::CornerCuttingStringency,
    experience_mode::ExperienceMode, flashback_limit::FlashbackLimit,
    gearbox_assist::GearboxAssist, low_fuel_mode::LowFuelMode, marshal_zone::MarshalZone,
    pit_stop_experience::PitStopExperience, race_starts::RaceStarts, recovery_mode::RecoveryMode,
    red_flags_setting::RedFlagsSetting, safety_car_setting::SafetyCarSetting,
    speed_units::SpeedUnits, surface_type::SurfaceType, temp_units::TempUnits, toggle::Toggle,
    tyre_temperature_mode::TyreTemperatureMode, weather_forecast_sample::WeatherForecastSample,
};
use crate::telemetry_data::{
    DynamicRacingLine, DynamicRacingLineType, Formula, GameMode, NetworkGame, RuleSet,
    SafetyCarStatus, SessionType, Track, Weather, packet_header::PacketHeader,
};

const WEATHER_FORECAST_SAMPLE_SIZE: usize = 64;

/// Session data packet containing information about the current session
///
/// This structure contains detailed information about the current session,
/// including weather conditions, track details, and game settings.
///
/// All fields use raw C types to match the UDP specification exactly.
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketSessionData {
    /// Header information for the packet
    pub header: PacketHeader,

    /// Weather - 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
    pub weather: Weather,

    /// Track temp in degrees Celsius
    pub track_temperature: i8,

    /// Air temp in degrees Celsius
    pub air_temperature: i8,

    /// Total number of laps in this race
    pub total_laps: u8,

    /// Track length in metres
    pub track_length: u16,

    /// 0 = unknown, see appendix
    pub session_type: SessionType,

    /// -1 for unknown, see appendix
    pub track_id: Track,

    /// Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2, 3 = F1 Generic, 4 = Beta, 6 = Esports, 8 = F1 World, 9 = F1 Elimination
    pub formula: Formula,

    /// Time left in session in seconds
    pub session_time_left: u16,

    /// Session duration in seconds
    pub session_duration: u16,

    /// Pit speed limit in kilometres per hour
    pub pit_speed_limit: u8,

    /// Whether the game is paused - network game only
    pub game_paused: bool,

    /// Whether the player is spectating
    pub is_spectating: bool,

    /// Index of the car being spectated
    #[serde(with = "crate::utils::u8_as_usize")]
    pub spectator_car_index: usize,

    /// SLI Pro support, 0 = inactive, 1 = active
    pub sli_pro_native_support: u8,

    /// Number of marshal zones to follow
    pub num_marshal_zones: u8,

    /// List of marshal zones - max 21
    pub marshal_zones: [MarshalZone; 21],

    /// 0 = no safety car, 1 = full, 2 = virtual, 3 = formation lap
    pub safety_car_status: SafetyCarStatus,

    /// 0 = offline, 1 = online
    pub network_game: NetworkGame,

    /// Number of weather samples to follow
    pub num_weather_forecast_samples: u8,

    /// Array of weather forecast samples
    #[serde(with = "BigArray")]
    pub weather_forecast_samples: [WeatherForecastSample; WEATHER_FORECAST_SAMPLE_SIZE],

    /// 0 = Perfect, 1 = Approximate
    pub forecast_accuracy: u8,

    /// AI difficulty - 0-110
    pub ai_difficulty: u8,

    /// Identifier for season - persists across saves
    pub season_link_identifier: u32,

    /// Identifier for weekend - persists across saves
    pub weekend_link_identifier: u32,

    /// Identifier for session - persists across saves
    pub session_link_identifier: u32,

    /// Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_ideal_lap: u8,

    /// Latest lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8,

    /// Predicted position to rejoin at (player)
    pub pit_stop_rejoin_position: u8,

    /// 0 = off, 1 = on
    pub steering_assist: Toggle,

    /// Braking assist level
    pub braking_assist: BrakingAssist,

    /// Gearbox assist setting
    pub gearbox_assist: GearboxAssist,

    /// 0 = off, 1 = on
    pub pit_assist: Toggle,

    /// 0 = off, 1 = on
    pub pit_release_assist: Toggle,

    /// 0 = off, 1 = on
    pub ers_assist: Toggle,

    /// 0 = off, 1 = on
    pub drs_assist: Toggle,

    /// 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line: DynamicRacingLine,

    /// 0 = 2D, 1 = 3D
    pub dynamic_racing_line_type: DynamicRacingLineType,

    /// Game mode id - see appendix
    pub game_mode: GameMode,

    /// Ruleset - see appendix
    pub rule_set: RuleSet,

    /// Local time of day - minutes since midnight
    pub time_of_day: u32,

    /// 0 = None, 2 = Very Short, 3 = Short, 4 = Medium, 5 = Medium Long, 6 = Long, 7 = Full
    pub session_length: u8,

    /// Speed units preference for lead player
    pub speed_units_lead_player: SpeedUnits,

    /// Temperature units preference for lead player
    pub temperature_units_lead_player: TempUnits,

    /// Speed units preference for secondary player
    pub speed_units_secondary_player: SpeedUnits,

    /// Temperature units preference for secondary player
    pub temperature_units_secondary_player: TempUnits,

    /// Number of safety cars called during session
    pub num_safety_car_periods: u8,

    /// Number of virtual safety cars called during session
    pub num_virtual_safety_car_periods: u8,

    /// Number of red flags called during session
    pub num_red_flag_periods: u8,

    /// Whether equal car performance is enabled
    pub equal_car_performance: Toggle,

    /// Recovery mode setting
    pub recovery_mode: RecoveryMode,

    /// Flashback limit setting
    pub flashback_limit: FlashbackLimit,

    /// Surface type simulation setting
    pub surface_type: SurfaceType,

    /// Low fuel mode setting
    pub low_fuel_mode: LowFuelMode,

    /// Race starts setting
    pub race_starts: RaceStarts,

    /// Tyre temperature simulation mode
    pub tyre_temperature: TyreTemperatureMode,

    /// Whether pit lane tyre simulation is disabled (0 = On, 1 = Off)
    pub pit_lane_tyre_sim: Toggle,

    /// Car damage setting
    pub car_damage: CarDamage,

    /// Car damage rate setting
    pub car_damage_rate: CarDamageRate,

    /// Collisions setting
    pub collisions: Collisions,

    /// Whether collisions off applies to first lap only
    pub collisions_off_for_first_lap_only: Toggle,

    /// Whether unsafe pit release is enabled in multiplayer (0 = On, 1 = Off)
    pub mp_unsafe_pit_release: Toggle,

    /// Whether collisions off for griefing is enabled in multiplayer
    pub mp_off_for_griefing: Toggle,

    /// Corner cutting stringency setting
    pub corner_cutting_stringency: CornerCuttingStringency,

    /// Whether parc fermé rules are enabled
    pub parc_ferme_rules: Toggle,

    /// Pit stop experience setting
    pub pit_stop_experience: PitStopExperience,

    /// Safety car frequency setting
    pub safety_car: SafetyCarSetting,

    /// Safety car experience mode
    pub safety_car_experience: ExperienceMode,

    /// Whether formation lap is enabled
    pub formation_lap: Toggle,

    /// Formation lap experience mode
    pub formation_lap_experience: ExperienceMode,

    /// Red flags frequency setting
    pub red_flags: RedFlagsSetting,

    /// Whether results affect licence level in solo play
    pub affects_licence_level_solo: Toggle,

    /// Whether results affect licence level in multiplayer
    pub affects_licence_level_mp: Toggle,

    /// Number of session in following array
    pub num_sessions_in_weekend: u8,

    /// List of session types to show weekend structure - see appendix for types
    pub weekend_structure: [SessionType; 12],

    /// Distance in m around track where sector 2 starts
    pub sector2_lap_distance_start: f32,

    /// Distance in m around track where sector 3 starts
    pub sector3_lap_distance_start: f32,
}

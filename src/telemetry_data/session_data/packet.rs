//! Defines the session data packet structure.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{
    CarDamage, CarDamageRate, Collisions, CornerCuttingStringency, ExperienceMode,
    FlashbackLimit, GameMode, LowFuelMode, PitStopExperience, RaceStarts, RecoveryMode,
    RedFlagsSetting, RuleSet, SafetyCarSetting, SpeedUnits, SurfaceType, TempUnits,
    TyreTemperatureMode, assist_toggle::AssistToggle, dynamic_racing_line::DynamicRacingLine,
    dynamic_racing_line_type::DynamicRacingLineType, forecast_accuracy::ForecastAccuracy,
    formula::Formula, gearbox_assist::GearboxAssist, marshal_zone::MarshalZone,
    network_game::NetworkGame, safety_car_status::SafetyCarStatus, session_type::SessionType,
    track::Track, weather::Weather, weather_forecast_sample::WeatherForecastSample,
};
use crate::telemetry_data::packet_header::PacketHeader;

const WEATHER_FORECAST_SAMPLE_SIZE: usize = 64;

/// Session data packet containing information about the current session
///
/// This structure contains detailed information about the current session,
/// including weather conditions, track details, and game settings.
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketSessionData {
    /// Header information for the packet
    pub header: PacketHeader,

    /// Current weather condition
    pub weather: Weather,

    /// Track temperature in degrees Celsius
    pub track_temperature: i8,

    /// Air temperature in degrees Celsius
    pub air_temperature: i8,

    /// Total number of laps in this race
    pub total_laps: u8,

    /// Track length in meters
    pub track_length: u16,

    /// Type of session (practice, qualifying, race)
    pub session_type: SessionType,

    /// Track identifier
    pub track: Track,

    /// Formula category
    pub formula: Formula,

    /// Time remaining in session (seconds)
    pub session_time_left: u16,

    /// Total session duration (seconds)
    pub session_duration: u16,

    /// Pit lane speed limit (km/h)
    pub pit_speed_limit: u8,

    /// Whether the game is currently paused
    pub game_paused: bool,

    /// Whether the player is spectating
    pub is_spectating: bool,

    /// Index of the car being spectated
    #[serde(with = "crate::utils::u8_as_usize")]
    pub spectator_car_index: usize,

    /// SLI Pro native support (0=inactive, 1=active)
    pub sli_pro_native_support: bool,

    /// Number of marshal zones on track
    pub num_marshal_zones: u8,

    /// List of marshal zones (max 21)
    pub marshal_zones: [MarshalZone; 21],

    /// Current safety car status
    pub safety_car_status: SafetyCarStatus,

    /// Network game status (offline/online)
    pub network_game: NetworkGame,

    /// Number of weather forecast samples
    pub num_weather_forecast_samples: u8,

    /// Array of weather forecast samples
    #[serde(with = "BigArray")]
    pub weather_forecast_samples: [WeatherForecastSample; WEATHER_FORECAST_SAMPLE_SIZE],

    /// Forecast accuracy (perfect/approximate)
    pub forecast_accuracy: ForecastAccuracy,

    /// AI difficulty rating (0-110)
    pub ai_difficulty: u8,

    /// Identifier for season (persists across saves)
    pub season_link_identifier: u32,

    /// Identifier for weekend (persists across saves)
    pub weekend_link_identifier: u32,

    /// Identifier for session (persists across saves)
    pub session_link_identifier: u32,

    /// Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_ideal_lap: u8,

    /// Latest lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8,

    /// Predicted position to rejoin at after pit (player)
    pub pit_stop_rejoin_position: u8,

    /// Steering assist setting
    pub steering_assist: AssistToggle,

    /// Braking assist setting
    pub braking_assist: AssistToggle,

    /// Gearbox assist setting
    pub gearbox_assist: GearboxAssist,

    /// Pit assist setting
    pub pit_assist: AssistToggle,

    /// Pit release assist setting
    pub pit_release_assist: AssistToggle,

    /// ERS assist setting
    pub ers_assist: AssistToggle,

    /// DRS assist setting
    pub drs_assist: AssistToggle,

    /// Dynamic racing line setting
    pub dynamic_racing_line: DynamicRacingLine,

    /// Dynamic racing line type (2D/3D)
    pub dynamic_racing_line_type: DynamicRacingLineType,

    pub game_mode: GameMode,

    pub rule_set: RuleSet,

    pub time_of_day: u32,

    pub session_length: u8,

    /// Speed units preference for lead player
    pub speed_units_lead_player: SpeedUnits,

    /// Temperature units preference for lead player
    pub temp_units_lead_player: TempUnits,

    /// Speed units preference for secondary player
    pub speed_units_secondary_player: SpeedUnits,

    /// Temperature units preference for secondary player
    pub temp_units_secondary_player: TempUnits,

    /// Number of safety car periods in this session
    pub num_safety_car_periods: u8,

    /// Number of virtual safety car periods in this session
    pub num_vsc_periods: u8,

    /// Number of red flag periods in this session
    pub num_red_flag_periods: u8,

    /// Whether equal car performance is enabled
    pub equal_car_performance: bool,

    /// Recovery mode setting for handling player mistakes
    pub recovery_mode: RecoveryMode,

    /// Flashback limit setting
    pub flashback_limit: FlashbackLimit,

    /// Surface type simulation setting
    pub surface_type: SurfaceType,

    /// Low fuel mode difficulty setting
    pub low_fuel_mode: LowFuelMode,

    /// Race starts assist setting
    pub race_starts: RaceStarts,

    /// Tyre temperature simulation mode
    pub tyre_temperature: TyreTemperatureMode,

    /// Whether pit lane tyre simulation is disabled (false = enabled, true = disabled)
    pub pit_lane_tyre_sim_disabled: bool,

    /// Car damage simulation level
    pub car_damage: CarDamage,

    /// Car damage accumulation rate
    pub car_damage_rate: CarDamageRate,

    /// Collisions setting
    pub collisions: Collisions,

    /// Whether collisions are only disabled for the first lap
    pub collisions_off_for_first_lap_only: bool,

    /// Whether multiplayer unsafe pit release is disabled (false = enabled, true = disabled)
    pub mp_unsafe_pit_release_disabled: bool,

    /// Whether multiplayer off for griefing is enabled
    pub mp_off_for_griefing: bool,

    /// Corner cutting stringency level
    pub corner_cutting_stringency: CornerCuttingStringency,

    /// Whether parc fermé rules are enabled
    pub parc_ferme_rules: bool,

    /// Pit stop experience presentation mode
    pub pit_stop_experience: PitStopExperience,

    /// Safety car deployment frequency setting
    pub safety_car: SafetyCarSetting,

    /// Safety car experience presentation mode
    pub safety_car_experience: ExperienceMode,

    /// Whether formation lap is enabled
    pub formation_lap: bool,

    /// Formation lap experience presentation mode
    pub formation_lap_experience: ExperienceMode,

    /// Red flags deployment frequency setting
    pub red_flags: RedFlagsSetting,

    /// Whether results affect licence level in solo play
    pub affects_licence_level_solo: bool,

    /// Whether results affect licence level in multiplayer
    pub affects_licence_level_mp: bool,

    #[serde(with = "crate::utils::u8_as_usize")]
    pub num_sessions_in_weekend: usize, // Number of sessions in the following array

    pub weekend_structure: [u8; 12], // List of session types to show weekend structure

    pub sector2_lap_distance_start: f32, // Distance in meters where sector 2 starts

    pub sector3_lap_distance_start: f32, // Distance in meters where sector 3 starts
}

impl Default for PacketSessionData {
    fn default() -> Self {
        Self {
            header: PacketHeader {
                packet_id: crate::telemetry_data::PacketId::SessionPacket,
                ..PacketHeader::default()
            },
            weather: Weather::Clear,
            track_temperature: 0,
            air_temperature: 0,
            total_laps: 0,
            track_length: 0,
            session_type: SessionType::Unknown,
            track: Track::Unknown,
            formula: Formula::F1Modern,
            session_time_left: 0,
            session_duration: 0,
            pit_speed_limit: 0,
            game_paused: false,
            is_spectating: false,
            spectator_car_index: 0,
            sli_pro_native_support: false,
            num_marshal_zones: 0,
            marshal_zones: [MarshalZone::default(); 21],
            safety_car_status: SafetyCarStatus::NoSafetyCar,
            network_game: NetworkGame::Offline,
            num_weather_forecast_samples: 0,
            weather_forecast_samples: [WeatherForecastSample::default(); WEATHER_FORECAST_SAMPLE_SIZE],
            forecast_accuracy: ForecastAccuracy::Approximate,
            ai_difficulty: 0,
            season_link_identifier: 0,
            weekend_link_identifier: 0,
            session_link_identifier: 0,
            pit_stop_window_ideal_lap: 0,
            pit_stop_window_latest_lap: 0,
            pit_stop_rejoin_position: 0,
            steering_assist: AssistToggle::Off,
            braking_assist: AssistToggle::Off,
            gearbox_assist: GearboxAssist::Manual,
            pit_assist: AssistToggle::Off,
            pit_release_assist: AssistToggle::Off,
            ers_assist: AssistToggle::Off,
            drs_assist: AssistToggle::Off,
            dynamic_racing_line: DynamicRacingLine::TwoD,
            dynamic_racing_line_type: DynamicRacingLineType::Off,
            game_mode: GameMode::GrandPrix,
            rule_set: RuleSet::Race,
            time_of_day: 0,
            session_length: 0,
            speed_units_lead_player: SpeedUnits::Kph,
            temp_units_lead_player: TempUnits::Celsius,
            speed_units_secondary_player: SpeedUnits::Kph,
            temp_units_secondary_player: TempUnits::Celsius,
            num_safety_car_periods: 0,
            num_vsc_periods: 0,
            num_red_flag_periods: 0,
            equal_car_performance: false,
            recovery_mode: RecoveryMode::None,
            flashback_limit: FlashbackLimit::Low,
            surface_type: SurfaceType::Simplified,
            low_fuel_mode: LowFuelMode::Easy,
            race_starts: RaceStarts::Manual,
            tyre_temperature: TyreTemperatureMode::SurfaceOnly,
            pit_lane_tyre_sim_disabled: false,
            car_damage: CarDamage::Off,
            car_damage_rate: CarDamageRate::Reduced,
            collisions: Collisions::Off,
            collisions_off_for_first_lap_only: false,
            mp_unsafe_pit_release_disabled: false,
            mp_off_for_griefing: false,
            corner_cutting_stringency: CornerCuttingStringency::Regular,
            parc_ferme_rules: false,
            pit_stop_experience: PitStopExperience::Automatic,
            safety_car: SafetyCarSetting::Off,
            safety_car_experience: ExperienceMode::Broadcast,
            formation_lap: false,
            formation_lap_experience: ExperienceMode::Broadcast,
            red_flags: RedFlagsSetting::Off,
            affects_licence_level_solo: false,
            affects_licence_level_mp: false,
            num_sessions_in_weekend: 0,
            weekend_structure: [0; 12],
            sector2_lap_distance_start: 0.0,
            sector3_lap_distance_start: 0.0,
        }
    }
}
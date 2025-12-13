//! Defines the session data packet structure.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{
    braking_assist::BrakingAssist, car_damage::CarDamage, car_damage_rate::CarDamageRate,
    collisions::Collisions, corner_cutting_stringency::CornerCuttingStringency,
    experience_mode::ExperienceMode, flashback_limit::FlashbackLimit, low_fuel_mode::LowFuelMode,
    marshal_zone::MarshalZone, pit_stop_experience::PitStopExperience, race_starts::RaceStarts,
    recovery_mode::RecoveryMode, red_flags_setting::RedFlagsSetting,
    safety_car_setting::SafetyCarSetting, speed_units::SpeedUnits, surface_type::SurfaceType,
    temp_units::TempUnits, toggle::Toggle, tyre_temperature_mode::TyreTemperatureMode,
    weather_forecast_sample::WeatherForecastSample,
};
use crate::telemetry_data::{
    AssistToggle, DynamicRacingLine, DynamicRacingLineType, ForecastAccuracy, Formula, GameMode,
    NetworkGame, RuleSet, SafetyCarStatus, SessionType, Track, Weather,
    packet_header::PacketHeader,
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

    /// Current weather conditions
    pub weather: Weather,

    /// Track temp in degrees Celsius
    pub track_temperature: i8,

    /// Air temp in degrees Celsius
    pub air_temperature: i8,

    /// Total number of laps in this race
    pub total_laps: u8,

    /// Track length in metres
    pub track_length: u16,

    /// Type of session (practice, qualifying, race, etc.)
    pub session_type: SessionType,

    /// Track identifier
    pub track_id: Track,

    /// Formula category being used
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

    /// Whether SLI Pro native support is active
    pub sli_pro_native_support: u8,

    /// Number of marshal zones to follow
    pub num_marshal_zones: u8,

    /// List of marshal zones - max 21
    pub marshal_zones: [MarshalZone; 21],

    /// Current safety car status
    pub safety_car_status: SafetyCarStatus,

    /// Whether this is a network game
    pub network_game: NetworkGame,

    /// Number of weather samples to follow
    pub num_weather_forecast_samples: u8,

    /// Array of weather forecast samples
    #[serde(with = "BigArray")]
    pub weather_forecast_samples: [WeatherForecastSample; WEATHER_FORECAST_SAMPLE_SIZE],

    /// Weather forecast accuracy level
    pub forecast_accuracy: ForecastAccuracy,

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

    /// Whether steering assist is enabled
    pub steering_assist: AssistToggle,

    /// Braking assist level
    pub braking_assist: BrakingAssist,

    /// Gearbox assist setting
    pub gearbox_assist: AssistToggle,

    /// Whether pit assist is enabled
    pub pit_assist: AssistToggle,

    /// Whether pit release assist is enabled
    pub pit_release_assist: AssistToggle,

    /// Whether ERS assist is enabled
    pub ers_assist: AssistToggle,

    /// Whether DRS assist is enabled
    pub drs_assist: AssistToggle,

    /// Dynamic racing line visibility setting
    pub dynamic_racing_line: DynamicRacingLine,

    /// Dynamic racing line display type
    pub dynamic_racing_line_type: DynamicRacingLineType,

    /// Game mode identifier
    pub game_mode: GameMode,

    /// Ruleset being used
    pub rule_set: RuleSet,

    /// Local time of day - minutes since midnight
    pub time_of_day: u32,

    /// Session length setting
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

    /// Whether pit lane tyre simulation is enabled
    pub pit_lane_tyre_sim: Toggle,

    /// Car damage setting
    pub car_damage: CarDamage,

    /// Car damage rate setting
    pub car_damage_rate: CarDamageRate,

    /// Collisions setting
    pub collisions: Collisions,

    /// Whether collisions off applies to first lap only
    pub collisions_off_for_first_lap_only: Toggle,

    /// Whether unsafe pit release is enabled in multiplayer
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
            track_id: Track::Unknown,
            formula: Formula::F1Modern,
            session_time_left: 0,
            session_duration: 0,
            pit_speed_limit: 0,
            game_paused: false,
            is_spectating: false,
            spectator_car_index: 0,
            sli_pro_native_support: 0,
            num_marshal_zones: 0,
            marshal_zones: [MarshalZone::default(); 21],
            safety_car_status: SafetyCarStatus::NoSafetyCar,
            network_game: NetworkGame::Offline,
            num_weather_forecast_samples: 0,
            weather_forecast_samples: [WeatherForecastSample::default();
                WEATHER_FORECAST_SAMPLE_SIZE],
            forecast_accuracy: ForecastAccuracy::Approximate,
            ai_difficulty: 0,
            season_link_identifier: 0,
            weekend_link_identifier: 0,
            session_link_identifier: 0,
            pit_stop_window_ideal_lap: 0,
            pit_stop_window_latest_lap: 0,
            pit_stop_rejoin_position: 0,
            steering_assist: AssistToggle::Off,
            braking_assist: BrakingAssist::default(),
            gearbox_assist: AssistToggle::Off,
            pit_assist: AssistToggle::Off,
            pit_release_assist: AssistToggle::Off,
            ers_assist: AssistToggle::Off,
            drs_assist: AssistToggle::Off,
            dynamic_racing_line: DynamicRacingLine::ThreeD,
            dynamic_racing_line_type: DynamicRacingLineType::CornersOnly,
            game_mode: GameMode::GrandPrix,
            rule_set: RuleSet::Race,
            time_of_day: 0,
            session_length: 0,
            speed_units_lead_player: SpeedUnits::Kph,
            temperature_units_lead_player: TempUnits::Celsius,
            speed_units_secondary_player: SpeedUnits::Kph,
            temperature_units_secondary_player: TempUnits::Celsius,
            num_safety_car_periods: 0,
            num_virtual_safety_car_periods: 0,
            num_red_flag_periods: 0,
            equal_car_performance: Toggle::Off,
            recovery_mode: RecoveryMode::AutoRecovery,
            flashback_limit: FlashbackLimit::High,
            surface_type: SurfaceType::Simplified,
            low_fuel_mode: LowFuelMode::Easy,
            race_starts: RaceStarts::Manual,
            tyre_temperature: TyreTemperatureMode::SurfaceOnly,
            pit_lane_tyre_sim: Toggle::Off,
            car_damage: CarDamage::Off,
            car_damage_rate: CarDamageRate::Reduced,
            collisions: Collisions::Off,
            collisions_off_for_first_lap_only: Toggle::Off,
            mp_unsafe_pit_release: Toggle::Off,
            mp_off_for_griefing: Toggle::Off,
            corner_cutting_stringency: CornerCuttingStringency::Regular,
            parc_ferme_rules: Toggle::Off,
            pit_stop_experience: PitStopExperience::Immersive,
            safety_car: SafetyCarSetting::Standard,
            safety_car_experience: ExperienceMode::Unknown,
            formation_lap: Toggle::Off,
            formation_lap_experience: ExperienceMode::Unknown,
            red_flags: RedFlagsSetting::Standard,
            affects_licence_level_solo: Toggle::Off,
            affects_licence_level_mp: Toggle::Off,
            num_sessions_in_weekend: 0,
            weekend_structure: [SessionType::Unknown; 12],
            sector2_lap_distance_start: 0.0,
            sector3_lap_distance_start: 0.0,
        }
    }
}

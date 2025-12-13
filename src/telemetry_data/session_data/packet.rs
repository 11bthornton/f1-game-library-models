//! Defines the session data packet structure.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{
    GameMode, RuleSet, assist_toggle::AssistToggle, dynamic_racing_line::DynamicRacingLine,
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
    pub num_marshall_zones: u8,

    /// List of marshal zones (max 21)
    pub marshall_zones: [MarshalZone; 21],

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

    /// Speed units for lead player (0=MPH, 1=KPH)
    pub speed_units_lead_player: u8,

    /// Temperature units for lead player (0=Celsius, 1=Fahrenheit)
    pub temp_units_lead_player: u8,

    /// Speed units for secondary player
    pub speed_units_secondary_player: u8,

    /// Temperature units for secondary player
    pub temp_units_secondary_player: u8,

    /// Number of safety car periods in this session
    pub num_safety_car_periods: u8,

    /// Number of virtual safety car periods in this session
    pub num_vsc_periods: u8,

    /// Number of red flag periods in this session
    pub num_red_flag_periods: u8,

    pub equal_car_performance: u8, // 0 = Off, 1 = On

    pub recovery_mode: u8, // 0 = None, 1 = Flashbacks, 2 = Auto-recovery

    pub flashback_limit: u8, // 0 = Low, 1 = Medium, 2 = High, 3 = Unlimited

    pub surface_type: u8, // 0 = Simplified, 1 = Realistic

    pub low_fuel_mode: u8, // 0 = Easy, 1 = Hard

    pub race_starts: u8, // 0 = Manual, 1 = Assisted

    pub tyre_temperature: u8, // 0 = Surface only, 1 = Surface & Carcass

    pub pit_lane_tyre_sim: u8, // 0 = On, 1 = Off

    pub car_damage: u8, // 0 = Off, 1 = Reduced, 2 = Standard, 3 = Simulation

    pub car_damage_rate: u8, // 0 = Reduced, 1 = Standard, 2 = Simulation

    pub collisions: u8, // 0 = Off, 1 = Player-to-Player Off, 2 = On

    pub collisions_off_for_first_lap_only: u8, // 0 = Disabled, 1 = Enabled

    pub mp_unsafe_pit_release: u8, // 0 = On, 1 = Off (Multiplayer)

    pub mp_off_for_griefing: u8, // 0 = Disabled, 1 = Enabled (Multiplayer)

    pub corner_cutting_stringency: u8, // 0 = Regular, 1 = Strict

    pub parc_ferme_rules: u8, // 0 = Off, 1 = On

    pub pit_stop_experience: u8, // 0 = Automatic, 1 = Broadcast, 2 = Immersive

    pub safety_car: u8, // 0 = Off, 1 = Reduced, 2 = Standard, 3 = Increased

    pub safety_car_experience: u8, // 0 = Broadcast, 1 = Immersive

    pub formation_lap: u8, // 0 = Off, 1 = On

    pub formation_lap_experience: u8, // 0 = Broadcast, 1 = Immersive

    pub red_flags: u8, // 0 = Off, 1 = Reduced, 2 = Standard, 3 = Increased

    pub affects_licence_level_solo: bool, // 0 = Off, 1 = On

    pub affects_licence_level_mp: bool, // 0 = Off, 1 = On

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
            num_marshall_zones: 0,
            marshall_zones: [MarshalZone::default(); 21],
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
            speed_units_lead_player: 1,
            temp_units_lead_player: 0,
            speed_units_secondary_player: 1,
            temp_units_secondary_player: 0,
            num_safety_car_periods: 0,
            num_vsc_periods: 0,
            num_red_flag_periods: 0,
            equal_car_performance: 0,
            recovery_mode: 0,
            flashback_limit: 0,
            surface_type: 0,
            low_fuel_mode: 0,
            race_starts: 0,
            tyre_temperature: 0,
            pit_lane_tyre_sim: 0,
            car_damage: 0,
            car_damage_rate: 0,
            collisions: 0,
            collisions_off_for_first_lap_only: 0,
            mp_unsafe_pit_release: 0,
            mp_off_for_griefing: 0,
            corner_cutting_stringency: 0,
            parc_ferme_rules: 0,
            pit_stop_experience: 0,
            safety_car: 0,
            safety_car_experience: 0,
            formation_lap: 0,
            formation_lap_experience: 0,
            red_flags: 0,
            affects_licence_level_solo: false,
            affects_licence_level_mp: false,
            num_sessions_in_weekend: 0,
            weekend_structure: [0; 12],
            sector2_lap_distance_start: 0.0,
            sector3_lap_distance_start: 0.0,
        }
    }
}
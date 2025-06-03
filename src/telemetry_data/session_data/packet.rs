use serde::{
    Deserialize,
    Serialize,
};
use serde_big_array::BigArray;

use super::{
    GameMode,
    RuleSet,
    assist_toggle::AssistToggle,
    dynamic_racing_line::DynamicRacingLine,
    dynamic_racing_line_type::DynamicRacingLineType,
    forecast_accuracy::ForecastAccuracy,
    formula::Formula,
    gearbox_assist::GearboxAssist,
    marshal_zone::MarshalZone,
    network_game::NetworkGame,
    safety_car_status::SafetyCarStatus,
    session_type::SessionType,
    track::Track,
    weather::Weather,
    weather_forecast_sample::WeatherForecastSample,
};
use crate::telemetry_data::packet_header::PacketHeader;

const WEATHER_FORECAST_SAMPLE_SIZE: usize = 64;

/// Session data packet containing information about the current session
///
/// This structure contains detailed information about the current session,
/// including weather conditions, track details, and game settings.
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketSessionData
{
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

    pub affects_licence_level_solo: u8, // 0 = Off, 1 = On

    pub affects_licence_level_mp: u8, // 0 = Off, 1 = On

    pub num_sessions_in_weekend: u8, // Number of sessions in the following array

    pub weekend_structure: [u8; 12], // List of session types to show weekend structure

    pub sector2_lap_distance_start: f32, // Distance in meters where sector 2 starts

    pub sector3_lap_distance_start: f32, // Distance in meters where sector 3 starts
}

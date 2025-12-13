//! Defines the session data packet structure.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{marshal_zone::MarshalZone, weather_forecast_sample::WeatherForecastSample};
use crate::telemetry_data::packet_header::PacketHeader;

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
    pub weather: u8,

    /// Track temp in degrees Celsius
    pub track_temperature: i8,

    /// Air temp in degrees Celsius
    pub air_temperature: i8,

    /// Total number of laps in this race
    pub total_laps: u8,

    /// Track length in metres
    pub track_length: u16,

    /// 0 = unknown, see appendix
    pub session_type: u8,

    /// -1 for unknown, see appendix
    pub track_id: i8,

    /// Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2, 3 = F1 Generic, 4 = Beta, 6 = Esports, 8 = F1 World, 9 = F1 Elimination
    pub formula: u8,

    /// Time left in session in seconds
    pub session_time_left: u16,

    /// Session duration in seconds
    pub session_duration: u16,

    /// Pit speed limit in kilometres per hour
    pub pit_speed_limit: u8,

    /// Whether the game is paused - network game only
    pub game_paused: u8,

    /// Whether the player is spectating
    pub is_spectating: u8,

    /// Index of the car being spectated
    pub spectator_car_index: u8,

    /// SLI Pro support, 0 = inactive, 1 = active
    pub sli_pro_native_support: u8,

    /// Number of marshal zones to follow
    pub num_marshal_zones: u8,

    /// List of marshal zones - max 21
    pub marshal_zones: [MarshalZone; 21],

    /// 0 = no safety car, 1 = full, 2 = virtual, 3 = formation lap
    pub safety_car_status: u8,

    /// 0 = offline, 1 = online
    pub network_game: u8,

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
    pub steering_assist: u8,

    /// 0 = off, 1 = low, 2 = medium, 3 = high
    pub braking_assist: u8,

    /// 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub gearbox_assist: u8,

    /// 0 = off, 1 = on
    pub pit_assist: u8,

    /// 0 = off, 1 = on
    pub pit_release_assist: u8,

    /// 0 = off, 1 = on
    pub ers_assist: u8,

    /// 0 = off, 1 = on
    pub drs_assist: u8,

    /// 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line: u8,

    /// 0 = 2D, 1 = 3D
    pub dynamic_racing_line_type: u8,

    /// Game mode id - see appendix
    pub game_mode: u8,

    /// Ruleset - see appendix
    pub rule_set: u8,

    /// Local time of day - minutes since midnight
    pub time_of_day: u32,

    /// 0 = None, 2 = Very Short, 3 = Short, 4 = Medium, 5 = Medium Long, 6 = Long, 7 = Full
    pub session_length: u8,

    /// 0 = MPH, 1 = KPH
    pub speed_units_lead_player: u8,

    /// 0 = Celsius, 1 = Fahrenheit
    pub temperature_units_lead_player: u8,

    /// 0 = MPH, 1 = KPH
    pub speed_units_secondary_player: u8,

    /// 0 = Celsius, 1 = Fahrenheit
    pub temperature_units_secondary_player: u8,

    /// Number of safety cars called during session
    pub num_safety_car_periods: u8,

    /// Number of virtual safety cars called during session
    pub num_virtual_safety_car_periods: u8,

    /// Number of red flags called during session
    pub num_red_flag_periods: u8,

    /// 0 = Off, 1 = On
    pub equal_car_performance: u8,

    /// 0 = None, 1 = Flashbacks, 2 = Auto-recovery
    pub recovery_mode: u8,

    /// 0 = Low, 1 = Medium, 2 = High, 3 = Unlimited
    pub flashback_limit: u8,

    /// 0 = Simplified, 1 = Realistic
    pub surface_type: u8,

    /// 0 = Easy, 1 = Hard
    pub low_fuel_mode: u8,

    /// 0 = Manual, 1 = Assisted
    pub race_starts: u8,

    /// 0 = Surface only, 1 = Surface & Carcass
    pub tyre_temperature: u8,

    /// 0 = On, 1 = Off
    pub pit_lane_tyre_sim: u8,

    /// 0 = Off, 1 = Reduced, 2 = Standard, 3 = Simulation
    pub car_damage: u8,

    /// 0 = Reduced, 1 = Standard, 2 = Simulation
    pub car_damage_rate: u8,

    /// 0 = Off, 1 = Player-to-Player Off, 2 = On
    pub collisions: u8,

    /// 0 = Disabled, 1 = Enabled
    pub collisions_off_for_first_lap_only: u8,

    /// 0 = On, 1 = Off (Multiplayer)
    pub mp_unsafe_pit_release: u8,

    /// 0 = Disabled, 1 = Enabled (Multiplayer)
    pub mp_off_for_griefing: u8,

    /// 0 = Regular, 1 = Strict
    pub corner_cutting_stringency: u8,

    /// 0 = Off, 1 = On
    pub parc_ferme_rules: u8,

    /// 0 = Automatic, 1 = Broadcast, 2 = Immersive
    pub pit_stop_experience: u8,

    /// 0 = Off, 1 = Reduced, 2 = Standard, 3 = Increased
    pub safety_car: u8,

    /// 0 = Broadcast, 1 = Immersive
    pub safety_car_experience: u8,

    /// 0 = Off, 1 = On
    pub formation_lap: u8,

    /// 0 = Broadcast, 1 = Immersive
    pub formation_lap_experience: u8,

    /// 0 = Off, 1 = Reduced, 2 = Standard, 3 = Increased
    pub red_flags: u8,

    /// 0 = Off, 1 = On
    pub affects_licence_level_solo: u8,

    /// 0 = Off, 1 = On
    pub affects_licence_level_mp: u8,

    /// Number of session in following array
    pub num_sessions_in_weekend: u8,

    /// List of session types to show weekend structure - see appendix for types
    pub weekend_structure: [u8; 12],

    /// Distance in m around track where sector 2 starts
    pub sector2_lap_distance_start: f32,

    /// Distance in m around track where sector 3 starts
    pub sector3_lap_distance_start: f32,
}
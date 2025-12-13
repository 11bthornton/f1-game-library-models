//! Session data from F1 games.
//!
//! This module contains structures for representing session information in F1 games,
//! including weather conditions, track details, and game settings.

mod assist_toggle;
mod car_damage;
mod car_damage_rate;
mod collisions;
mod corner_cutting_stringency;
mod dynamic_racing_line;
mod dynamic_racing_line_type;
mod experience_mode;
mod flashback_limit;
mod forecast_accuracy;
mod formula;
mod game_mode;
mod gearbox_assist;
mod low_fuel_mode;
mod marshal_zone;
mod network_game;
mod packet;
mod pit_stop_experience;
mod race_starts;
mod recovery_mode;
mod red_flags_setting;
mod rule_set;
mod safety_car_setting;
mod safety_car_status;
mod session_type;
mod speed_units;
mod surface_type;
mod temp_units;
mod track;
mod tyre_temperature_mode;
mod weather;
mod weather_forecast_sample;

// Re-export the structs and enums
pub use assist_toggle::AssistToggle;
pub use car_damage::CarDamage;
pub use car_damage_rate::CarDamageRate;
pub use collisions::Collisions;
pub use corner_cutting_stringency::CornerCuttingStringency;
pub use dynamic_racing_line::DynamicRacingLine;
pub use dynamic_racing_line_type::DynamicRacingLineType;
pub use experience_mode::ExperienceMode;
pub use flashback_limit::FlashbackLimit;
pub use forecast_accuracy::ForecastAccuracy;
pub use formula::Formula;
pub use game_mode::GameMode;
pub use gearbox_assist::GearboxAssist;
pub use low_fuel_mode::LowFuelMode;
pub use marshal_zone::MarshalZone;
pub use network_game::NetworkGame;
pub use packet::PacketSessionData;
pub use pit_stop_experience::PitStopExperience;
pub use race_starts::RaceStarts;
pub use recovery_mode::RecoveryMode;
pub use red_flags_setting::RedFlagsSetting;
pub use rule_set::RuleSet;
pub use safety_car_setting::SafetyCarSetting;
pub use safety_car_status::SafetyCarStatus;
pub use session_type::SessionType;
pub use speed_units::SpeedUnits;
pub use surface_type::SurfaceType;
pub use temp_units::TempUnits;
pub use track::Track;
pub use tyre_temperature_mode::TyreTemperatureMode;
pub use weather::Weather;
pub use weather_forecast_sample::WeatherForecastSample;

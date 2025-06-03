/// Session data from F1 games.
///
/// This module contains structures for representing session information in F1 games,
/// including weather conditions, track details, and game settings.
mod assist_toggle;
mod dynamic_racing_line;
mod dynamic_racing_line_type;
mod forecast_accuracy;
mod formula;
mod game_mode;
mod gearbox_assist;
mod marshal_zone;
mod network_game;
mod packet;
mod rule_set;
mod safety_car_status;
mod session_type;
mod track;
mod weather;
mod weather_forecast_sample;

// Re-export the structs and enums
pub use assist_toggle::AssistToggle;
pub use dynamic_racing_line::DynamicRacingLine;
pub use dynamic_racing_line_type::DynamicRacingLineType;
pub use forecast_accuracy::ForecastAccuracy;
pub use formula::Formula;
pub use game_mode::GameMode;
pub use gearbox_assist::GearboxAssist;
pub use marshal_zone::MarshalZone;
pub use network_game::NetworkGame;
pub use packet::PacketSessionData;
pub use rule_set::RuleSet;
pub use safety_car_status::SafetyCarStatus;
pub use session_type::SessionType;
pub use track::Track;
pub use weather::Weather;
pub use weather_forecast_sample::WeatherForecastSample;

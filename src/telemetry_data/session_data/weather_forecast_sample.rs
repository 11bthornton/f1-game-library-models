use serde::{Deserialize, Serialize};

use super::{session_type::SessionType, weather::Weather};

/// Weather forecast data for a specific time point
///
/// Contains predicted weather conditions and temperatures
#[derive(Deserialize, Debug, Serialize, Default, Copy, Clone, PartialEq)]
pub struct WeatherForecastSample {
    /// Session type this forecast is for
    pub session_type: SessionType,

    /// Time in minutes the forecast is for
    pub time_offset: u8,

    /// Weather condition predicted
    pub weather: Weather,

    /// Predicted track temperature in degrees Celsius
    pub track_temperature: i8,

    /// Predicted track temperature change
    pub track_temperature_change: i8,

    /// Predicted air temperature in degrees Celsius
    pub air_temperature: i8,

    /// Predicted air temperature change
    pub air_temperature_change: i8,

    /// Probability of rain as a percentage
    pub rain_percentage: u8,
}

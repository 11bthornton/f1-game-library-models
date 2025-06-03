use serde::{Deserialize, Serialize};

use crate::telemetry_data::car_status_data;

/// Historical data for a single tyre stint
///
/// Contains information about a tyre stint, including when it ended
/// and which compounds were used
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct TyreStintHistoryData {
    /// Lap number when this stint ended
    pub end_lap: u8,
    /// Actual tyre compound used in this stint
    pub tyre_actual_compound: car_status_data::ActualTyreCompound,
    /// Visual tyre compound displayed in this stint
    pub tyre_visual_compound: car_status_data::VisualTyreCompound,
}

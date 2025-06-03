use serde::{
    Deserialize,
    Serialize,
};

/// Data for a single tyre set.
///
/// This structure contains detailed information about a single tyre set,
/// including compound type, wear, and availability.
///
/// # Fields
///
/// * `actual_tyre_compound` - Actual tyre compound used
/// * `visual_tyre_compound` - Visual tyre compound used
/// * `wear` - Tyre wear (percentage)
/// * `available` - Whether this set is currently available
/// * `recommended_session` - Recommended session for tyre set
/// * `life_span` - Laps left in this tyre set
/// * `usable_life` - Max number of laps recommended for this compound
/// * `lap_delta_time` - Lap delta time in milliseconds compared to fitted set
/// * `fitted` - Whether the set is fitted or not
#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct TyreSetData
{
    /// Actual tyre compound used
    pub actual_tyre_compound: crate::telemetry_data::car_status_data::ActualTyreCompound,
    /// Visual tyre compound used
    pub visual_tyre_compound: crate::telemetry_data::car_status_data::VisualTyreCompound,
    /// Tyre wear (percentage)
    pub wear: u8,
    /// Whether this set is currently available
    pub available: bool,
    /// Recommended session for tyre set
    pub recommended_session: u8,
    /// Laps left in this tyre set
    pub life_span: u8,
    /// Max number of laps recommended for this compound
    pub usable_life: u8,
    /// Lap delta time in milliseconds compared to fitted set
    pub lap_delta_time: i16,
    /// Whether the set is fitted or not
    pub fitted: bool,
}

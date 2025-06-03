use serde::{Deserialize, Serialize};

/// Wing damage data.
///
/// This structure contains damage information for the front and rear wings.
///
/// # Fields
///
/// * `front_left` - Front left wing damage percentage
/// * `front_right` - Front right wing damage percentage
/// * `rear` - Rear wing damage percentage
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct WingDamage {
    /// Front left wing damage percentage
    pub front_left: u8,
    /// Front right wing damage percentage
    pub front_right: u8,
    /// Rear wing damage percentage
    pub rear: u8,
}

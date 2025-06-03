use serde_repr::{Deserialize_repr, Serialize_repr};

/// Weather forecast accuracy level
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ForecastAccuracy {
    /// Approximate forecast (may change)
    Approximate,
    /// Perfect forecast (won't change)
    Perfect,
}

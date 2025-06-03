use serde::{Deserialize, Serialize};

/// Engine component wear data.
///
/// This structure contains wear information for various engine components.
///
/// # Fields
///
/// * `mguh` - MGU-H wear percentage
/// * `es` - Energy Store wear percentage
/// * `ce` - Control Electronics wear percentage
/// * `ice` - Internal Combustion Engine wear percentage
/// * `mguk` - MGU-K wear percentage
/// * `tc` - Turbocharger wear percentage
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct EngineWear {
    /// MGU-H wear percentage
    pub mguh: u8,
    /// Energy Store wear percentage
    pub es: u8,
    /// Control Electronics wear percentage
    pub ce: u8,
    /// Internal Combustion Engine wear percentage
    pub ice: u8,
    /// MGU-K wear percentage
    pub mguk: u8,
    /// Turbocharger wear percentage
    pub tc: u8,
}

use serde::{
    Deserialize,
    Serialize,
};

/// ERS deployment modes.
///
/// These modes control how the Energy Recovery System (ERS) is deployed.
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(u8)]
pub enum DeployMode
{
    /// No ERS deployment
    None,
    /// Medium ERS deployment
    Medium,
    /// Maximum ERS deployment for hotlap
    Hotlap,
    /// Maximum ERS deployment for overtaking
    Overtake,
}

use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Safety car status in the current session
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum SafetyCarStatus
{
    /// No safety car on track
    #[serde(rename = "No Safety Car")]
    NoSafetyCar,
    /// Full safety car deployed
    #[serde(rename = "Full Safety Car")]
    FullSafetyCar,
    /// Virtual safety car deployed
    #[serde(rename = "Virtual Safety Car")]
    VirtualSafetyCar,
    /// Cars on formation lap
    #[serde(rename = "Formation Lap")]
    FormationLap,
}

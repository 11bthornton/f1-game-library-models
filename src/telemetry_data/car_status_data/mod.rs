//! Car status data from F1 games.
//!
//! This module contains structures for representing car status information in F1 games,
//! including driver aids, tyre compounds, and vehicle flags.

mod car_status_data;
mod driver_aids;
mod ers;
mod packet;
mod tyre_compounds;
mod vehicle_flags;

pub use car_status_data::CarStatusData;
pub use driver_aids::{AntiLockBrakes, FuelMix, PitLimiterStatus, TractionControl};
pub use ers::DeployMode;
pub use packet::PacketCarStatusData;
pub use tyre_compounds::{ActualTyreCompound, VisualTyreCompound};
pub use vehicle_flags::VehicleFiaFlags;

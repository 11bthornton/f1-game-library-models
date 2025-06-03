/// Car damage data components for F1 games.
///
/// This module contains structures for representing damage information components
/// for cars in F1 games.
mod brake_data;
mod car_data;
mod engine_wear;
mod packet;
mod tyre_data;
mod wing_damage;

pub use brake_data::BrakeData;
pub use car_data::CarDamageData;
pub use engine_wear::EngineWear;
pub use packet::PacketCarDamageData;
pub use tyre_data::TyreData;
pub use wing_damage::WingDamage;

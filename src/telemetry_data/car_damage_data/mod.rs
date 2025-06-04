/// Car damage data components for F1 games.
///
/// This module contains structures for representing damage information components
/// for cars in F1 games.
mod car_data;
mod engine_wear;
mod packet;
mod wing_damage;

pub use car_data::CarDamageData;
pub use engine_wear::EngineWear;
pub use packet::PacketCarDamageData;
pub use wing_damage::WingDamage;

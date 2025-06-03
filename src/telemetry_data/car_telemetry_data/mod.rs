/// Car telemetry data from F1 games.
///
/// This module contains structures for representing real-time telemetry data
/// from cars in F1 games, including speed, throttle, brake, gear, engine RPM,
/// and various temperature readings.
mod car_telemetry;
mod packet;

// Re-export the structs
pub use car_telemetry::CarTelemetryData;
pub use packet::PacketCarTelemetryData;

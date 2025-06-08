//! Defines the `LapData` structure which contains detailed lap timing and position information

use serde::{Deserialize, Serialize};

use super::{
    driver_status::DriverStatus, pit_status::PitStatus, result_status::ResultStatus, sector::Sector,
};

/// Lap data for a single car.
///
/// This structure contains detailed lap timing and position information
/// for a single car during a session.
///
/// # Fields
///
/// * `last_lap_time_in_ms` - Last completed lap time in milliseconds
/// * `current_lap_time_in_ms` - Current lap time in milliseconds
/// * `sector1_time_in_ms` - Sector 1 time in milliseconds
/// * `sector1_time_minutes` - Sector 1 whole minute part
/// * `sector2_time_in_ms` - Sector 2 time in milliseconds
/// * `sector2_time_minutes` - Sector 2 whole minute part
/// * `delta_to_car_in_front_in_ms` - Time delta to car in front in milliseconds
/// * `delta_to_race_leader_in_ms` - Time delta to race leader in milliseconds
/// * `lap_distance` - Distance around current lap in meters
/// * `total_distance` - Total distance traveled in session in meters
/// * `safety_car_delta` - Delta in seconds for safety car
/// * `car_position` - Car's current race position
/// * `current_lap_num` - Current lap number
/// * `pit_status` - Current pit status
/// * `num_pit_stops` - Number of pit stops taken in this race
/// * `sector` - Current sector the car is in
/// * `current_lap_invalid` - Whether current lap is invalid
/// * `penalties` - Accumulated time penalties in seconds
/// * `total_warnings` - Accumulated number of warnings issued
/// * `corner_cutting_warnings` - Accumulated number of corner cutting warnings
/// * `num_unserved_drive_through_pens` - Number of unserved drive-through penalties
/// * `num_unserved_stop_go_pens` - Number of unserved stop-go penalties
/// * `grid_position` - Grid position the vehicle started the race in
/// * `driver_status` - Status of driver (in garage, on track, etc.)
/// * `result_status` - Result status (active, finished, DNF, etc.)
/// * `pit_lane_timer_active` - Whether pit lane timer is active
/// * `pit_lane_time_in_lane_in_ms` - Time spent in pit lane in milliseconds
/// * `pit_stop_timer_in_ms` - Time of the actual pit stop in milliseconds
/// * `pit_stop_should_serve_pen` - Whether the car should serve a penalty at this stop
#[derive(Deserialize, Debug, Default, Serialize, Clone, Copy, PartialEq)]
pub struct LapData {
    /// Last completed lap time in milliseconds
    pub last_lap_time_in_ms: u32,
    /// Current lap time in milliseconds
    pub current_lap_time_in_ms: u32,
    /// Sector 1 time in milliseconds
    pub sector1_time_in_ms: u16,
    /// Sector 1 whole minute part
    pub sector1_time_minutes: u8,
    /// Sector 2 time in milliseconds
    pub sector2_time_in_ms: u16,
    /// Sector 2 whole minute part
    pub sector2_time_minutes: u8,
    /// Time delta to car in front in milliseconds
    pub delta_to_car_in_front_in_ms: u16,
    /// Time delta to car in front in minutes part
    pub delta_to_car_in_front_minutes_part: u8,
    /// Time delta to race leader in milliseconds
    pub delta_to_race_leader_in_ms: u16,
    /// Time to race leader in minutes part
    pub delta_to_race_leader_in_front_minutes_part: u8,
    /// Distance around current lap in meters (can be negative if line hasn't been crossed yet)
    pub lap_distance: f32,
    /// Total distance traveled in session in meters (can be negative if line hasn't been crossed
    /// yet)
    pub total_distance: f32,
    /// Delta in seconds for safety car
    pub safety_car_delta: f32,
    /// Car's current race position
    pub car_position: u8,
    /// Current lap number
    pub current_lap_num: u8,
    /// Current pit status (none, pitting, in pit area)
    pub pit_status: PitStatus,
    /// Number of pit stops taken in this race
    pub num_pit_stops: u8,
    /// Current sector the car is in (sector1, sector2, sector3)
    pub sector: Sector,
    /// Whether current lap is invalid (0 = valid, 1 = invalid)
    pub current_lap_invalid: bool,
    /// Accumulated time penalties in seconds
    pub penalties: u8,
    /// Accumulated number of warnings issued
    pub total_warnings: u8,
    /// Accumulated number of corner cutting warnings issued
    pub corner_cutting_warnings: u8,
    /// Number of unserved drive-through penalties
    pub num_unserved_drive_through_pens: u8,
    /// Number of unserved stop-go penalties
    pub num_unserved_stop_go_pens: u8,
    /// Grid position the vehicle started the race in
    pub grid_position: u8,
    /// Status of driver (in garage, flying lap, in lap, out lap, on track)
    pub driver_status: DriverStatus,
    /// Result status (invalid, inactive, active, finished, DNF, DSQ, not classified, retired)
    pub result_status: ResultStatus,
    /// Whether pit lane timer is active (false = inactive, true = active)
    pub pit_lane_timer_active: bool,
    /// If pit lane timer is active, the current time spent in the pit lane in milliseconds
    pub pit_lane_time_in_lane_in_ms: u16,
    /// Time of the actual pit stop in milliseconds
    pub pit_stop_timer_in_ms: u16,
    /// Whether the car should serve a penalty at this stop (0 = no, 1 = yes)
    pub pit_stop_should_serve_pen: bool,
    /// Fastest speed achieved in the speed trap
    pub speed_trap_fastest_speed: f32,
    /// Lap number of the fastest speed trap
    pub speed_trap_fastest_lap: u8,
}

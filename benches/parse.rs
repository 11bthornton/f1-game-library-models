#![allow(dead_code)]

use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use criterion::{BenchmarkGroup, Criterion, black_box, criterion_group, criterion_main, measurement::WallTime};
use f1_game_library_models_25::parse;
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Bincode mirror structs
//
// Field-for-field replicas of the wire structs so that bincode (little-endian,
// no length prefixes on fixed-size arrays) reads the same bytes as
// ptr::read_unaligned. No serde dependency leaks into the main crate.
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct BincodeHeader {
    packet_format: u16,
    game_year: u8,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    overall_frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
}

// --- TimeTrial (101 bytes) --------------------------------------------------

#[derive(Deserialize)]
struct BincodeTimeTrialDataSet {
    car_idx: u8,
    team_id: u8,
    lap_time_in_ms: u32,
    sector1_time_in_ms: u32,
    sector2_time_in_ms: u32,
    sector3_time_in_ms: u32,
    traction_control: u8,
    gearbox_assist: u8,
    anti_lock_brakes: u8,
    equal_car_performance: u8,
    custom_setup: u8,
    valid: u8,
}

#[derive(Deserialize)]
struct BincodePacketTimeTrial {
    header: BincodeHeader,
    player_session_best: BincodeTimeTrialDataSet,
    personal_best: BincodeTimeTrialDataSet,
    rival: BincodeTimeTrialDataSet,
}

// --- LapData (1285 bytes) ---------------------------------------------------

#[derive(Deserialize)]
struct BincodeLapData {
    last_lap_time_in_ms: u32,
    current_lap_time_in_ms: u32,
    sector1_time_in_ms: u16,
    sector1_time_minutes: u8,
    sector2_time_in_ms: u16,
    sector2_time_minutes: u8,
    delta_to_car_in_front_in_ms: u16,
    delta_to_car_in_front_minutes: u8,
    delta_to_race_leader_in_ms: u16,
    delta_to_race_leader_minutes: u8,
    lap_distance: f32,
    total_distance: f32,
    safety_car_delta: f32,
    car_position: u8,
    current_lap_num: u8,
    pit_status: u8,
    num_pit_stops: u8,
    sector: u8,
    current_lap_invalid: u8,
    penalties: u8,
    total_warnings: u8,
    corner_cutting_warnings: u8,
    num_unserved_drive_through_pens: u8,
    num_unserved_stop_go_pens: u8,
    grid_position: u8,
    driver_status: u8,
    result_status: u8,
    pit_lane_timer_active: u8,
    pit_lane_time_in_lane_in_ms: u16,
    pit_stop_timer_in_ms: u16,
    pit_stop_should_serve_pen: u8,
    speed_trap_fastest_speed: f32,
    speed_trap_fastest_lap: u8,
}

#[derive(Deserialize)]
struct BincodePacketLapData {
    header: BincodeHeader,
    lap_data: [BincodeLapData; 22],
    time_trial_pb_car_idx: u8,
    time_trial_rival_car_idx: u8,
}

// --- CarTelemetry (1352 bytes) ----------------------------------------------

#[derive(Deserialize)]
struct BincodeWheelDataU16 { rear_left: u16, rear_right: u16, front_left: u16, front_right: u16 }
#[derive(Deserialize)]
struct BincodeWheelDataU8  { rear_left: u8,  rear_right: u8,  front_left: u8,  front_right: u8  }
#[derive(Deserialize)]
struct BincodeWheelDataF32 { rear_left: f32, rear_right: f32, front_left: f32, front_right: f32 }

#[derive(Deserialize)]
struct BincodeCarTelemetryData {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8,
    engine_rpm: u16,
    drs: u8,
    rev_lights_percent: u8,
    rev_lights_bit_value: u16,
    brakes_temperature: BincodeWheelDataU16,
    tyres_surface_temperature: BincodeWheelDataU8,
    tyres_inner_temperature: BincodeWheelDataU8,
    engine_temperature: u16,
    tyres_pressure: BincodeWheelDataF32,
    surface_type_raw: BincodeWheelDataU8,
}

#[derive(Deserialize)]
struct BincodePacketCarTelemetry {
    header: BincodeHeader,
    car_telemetry_data: [BincodeCarTelemetryData; 22],
    mfd_panel_index: u8,
    mfd_panel_index_secondary_player: u8,
    suggested_gear: i8,
}

// ---------------------------------------------------------------------------
// Bytemuck mirror structs
//
// repr(C, packed) + Pod/Zeroable lets bytemuck::pod_read_unaligned cast the
// raw buffer to the struct in one shot — the safe-Rust equivalent of our
// ptr::read_unaligned approach.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmHeader {
    packet_format: u16,
    game_year: u8,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    overall_frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
}

// --- TimeTrial (101 bytes) --------------------------------------------------

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmTimeTrialDataSet {
    car_idx: u8,
    team_id: u8,
    lap_time_in_ms: u32,
    sector1_time_in_ms: u32,
    sector2_time_in_ms: u32,
    sector3_time_in_ms: u32,
    traction_control: u8,
    gearbox_assist: u8,
    anti_lock_brakes: u8,
    equal_car_performance: u8,
    custom_setup: u8,
    valid: u8,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmPacketTimeTrial {
    header: BmHeader,
    player_session_best: BmTimeTrialDataSet,
    personal_best: BmTimeTrialDataSet,
    rival: BmTimeTrialDataSet,
}

// --- LapData (1285 bytes) ---------------------------------------------------

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmLapData {
    last_lap_time_in_ms: u32,
    current_lap_time_in_ms: u32,
    sector1_time_in_ms: u16,
    sector1_time_minutes: u8,
    sector2_time_in_ms: u16,
    sector2_time_minutes: u8,
    delta_to_car_in_front_in_ms: u16,
    delta_to_car_in_front_minutes: u8,
    delta_to_race_leader_in_ms: u16,
    delta_to_race_leader_minutes: u8,
    lap_distance: f32,
    total_distance: f32,
    safety_car_delta: f32,
    car_position: u8,
    current_lap_num: u8,
    pit_status: u8,
    num_pit_stops: u8,
    sector: u8,
    current_lap_invalid: u8,
    penalties: u8,
    total_warnings: u8,
    corner_cutting_warnings: u8,
    num_unserved_drive_through_pens: u8,
    num_unserved_stop_go_pens: u8,
    grid_position: u8,
    driver_status: u8,
    result_status: u8,
    pit_lane_timer_active: u8,
    pit_lane_time_in_lane_in_ms: u16,
    pit_stop_timer_in_ms: u16,
    pit_stop_should_serve_pen: u8,
    speed_trap_fastest_speed: f32,
    speed_trap_fastest_lap: u8,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmPacketLapData {
    header: BmHeader,
    lap_data: [BmLapData; 22],
    time_trial_pb_car_idx: u8,
    time_trial_rival_car_idx: u8,
}

// --- CarTelemetry (1352 bytes) ----------------------------------------------

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmWheelDataU16 { rear_left: u16, rear_right: u16, front_left: u16, front_right: u16 }
#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmWheelDataU8  { rear_left: u8,  rear_right: u8,  front_left: u8,  front_right: u8  }
#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmWheelDataF32 { rear_left: f32, rear_right: f32, front_left: f32, front_right: f32 }

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmCarTelemetryData {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8,
    engine_rpm: u16,
    drs: u8,
    rev_lights_percent: u8,
    rev_lights_bit_value: u16,
    brakes_temperature: BmWheelDataU16,
    tyres_surface_temperature: BmWheelDataU8,
    tyres_inner_temperature: BmWheelDataU8,
    engine_temperature: u16,
    tyres_pressure: BmWheelDataF32,
    surface_type_raw: BmWheelDataU8,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C, packed)]
struct BmPacketCarTelemetry {
    header: BmHeader,
    car_telemetry_data: [BmCarTelemetryData; 22],
    mfd_panel_index: u8,
    mfd_panel_index_secondary_player: u8,
    suggested_gear: i8,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_buf(packet_id: u8, size: usize) -> Vec<u8> {
    let mut buf = vec![0u8; size];
    buf[6] = packet_id;
    buf
}

fn add_triple<Bm, Bc, F>(group: &mut BenchmarkGroup<WallTime>, name: &str, buf: &[u8], bincode_fn: F)
where
    Bm: Pod,
    F: Fn(&[u8]) -> Result<Bc, Box<bincode::ErrorKind>>,
{
    group.bench_function(format!("{name}/ptr_read_unaligned"), |b| {
        b.iter(|| parse::parse(black_box(buf)))
    });
    group.bench_function(format!("{name}/bytemuck"), |b| {
        b.iter(|| bytemuck::pod_read_unaligned::<Bm>(black_box(&buf[..size_of::<Bm>()])))
    });
    group.bench_function(format!("{name}/bincode"), |b| {
        b.iter(|| bincode_fn(black_box(buf)))
    });
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_parse(c: &mut Criterion) {
    let time_trial    = make_buf(14, 101);
    let lap_data      = make_buf(2, 1285);
    let car_telemetry = make_buf(6, 1352);

    let mut g = c.benchmark_group("parse");

    add_triple::<BmPacketTimeTrial, BincodePacketTimeTrial, _>(
        &mut g, "time_trial_101b", &time_trial,
        |b| bincode::deserialize(b),
    );
    add_triple::<BmPacketLapData, BincodePacketLapData, _>(
        &mut g, "lap_data_1285b", &lap_data,
        |b| bincode::deserialize(b),
    );
    add_triple::<BmPacketCarTelemetry, BincodePacketCarTelemetry, _>(
        &mut g, "car_telemetry_1352b", &car_telemetry,
        |b| bincode::deserialize(b),
    );

    g.finish();
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);

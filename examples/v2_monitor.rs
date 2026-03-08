//! Minimal v2 packet monitor.
//!
//! Listens on UDP (default port 20777), prints a summary line for every packet,
//! and saves the raw bytes of the first occurrence of each packet type to
//! `test_packets/<PacketType>.bin` so they can be replayed in unit tests.
//!
//! Usage:
//!   cargo run --example v2_monitor
//!   cargo run --example v2_monitor -- 20777
//!   cargo run --example v2_monitor -- 20777 ./my_fixtures

use std::collections::HashSet;
use std::fs;
use std::net::UdpSocket;
use std::path::{Path, PathBuf};

use f1_game_library_models_25::parse::{self, V2Packet};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20777);
    let fixture_dir = PathBuf::from(args.get(2).map(String::as_str).unwrap_or("test_packets"));

    fs::create_dir_all(&fixture_dir).expect("could not create fixture directory");

    let addr = format!("0.0.0.0:{port}");
    let socket = UdpSocket::bind(&addr).expect("failed to bind socket");
    println!(
        "listening on {addr}  (saving new packet types to {})",
        fixture_dir.display()
    );
    println!("{:<22} {:>10}  notes", "packet type", "frame");
    println!("{}", "-".repeat(60));

    let mut buf = [0u8; 2048];
    let mut seen: HashSet<&'static str> = HashSet::new();

    loop {
        let (len, _src) = match socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("recv error: {e}");
                continue;
            }
        };
        let raw = &buf[..len];

        let packet = match parse::parse(raw) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("parse error: {e}");
                continue;
            }
        };

        let (type_name, frame, notes) = summarise(&packet);
        println!("{:<22} {:>10}  {}", type_name, frame, notes);

        if !seen.contains(type_name) {
            seen.insert(type_name);
            let path = fixture_dir.join(format!("{type_name}.bin"));
            save_fixture(&path, raw);
            println!("  → saved fixture: {}", path.display());
        }
    }
}

fn save_fixture(path: &Path, bytes: &[u8]) {
    if let Err(e) = fs::write(path, bytes) {
        eprintln!("  ! could not save fixture {}: {e}", path.display());
    }
}

/// Return (type_name, frame_id, short notes string) for display.
fn summarise(packet: &V2Packet) -> (&'static str, u32, String) {
    match packet {
        V2Packet::Motion(p) => ("Motion", p.header.frame_identifier, String::new()),
        V2Packet::Session(p) => (
            "Session",
            p.header.frame_identifier,
            format!(
                "track={} laps={} type={}",
                p.payload.track_id, p.payload.total_laps, p.payload.session_type,
            ),
        ),
        V2Packet::LapData(p) => {
            let car = p.header.player_car_index();
            let ld = &p.payload.lap_data[car];
            (
                "LapData",
                p.header.frame_identifier,
                format!("lap={} pos={}", ld.current_lap_num, ld.car_position),
            )
        }
        V2Packet::Event(p) => (
            "Event",
            p.header.frame_identifier,
            format!("code={}", String::from_utf8_lossy(&p.payload.event_string_code)),
        ),
        V2Packet::Participants(p) => (
            "Participants",
            p.header.frame_identifier,
            format!("num_active={}", p.payload.num_active_cars),
        ),
        V2Packet::CarSetups(p) => ("CarSetups", p.header.frame_identifier, String::new()),
        V2Packet::CarTelemetry(p) => {
            let car = p.header.player_car_index();
            let ct = p.payload.car_telemetry_data[car];
            let (speed, gear, throttle) = (ct.speed, ct.gear, ct.throttle);
            (
                "CarTelemetry",
                p.header.frame_identifier,
                format!("speed={speed} gear={gear} throttle={throttle:.2}"),
            )
        }
        V2Packet::CarStatus(p) => ("CarStatus", p.header.frame_identifier, String::new()),
        V2Packet::FinalClassification(p) => (
            "FinalClassification",
            p.header.frame_identifier,
            format!("num_cars={}", p.payload.num_cars),
        ),
        V2Packet::LobbyInfo(p) => (
            "LobbyInfo",
            p.header.frame_identifier,
            format!("num_players={}", p.payload.num_players),
        ),
        V2Packet::CarDamage(p) => ("CarDamage", p.header.frame_identifier, String::new()),
        V2Packet::SessionHistory(p) => (
            "SessionHistory",
            p.header.frame_identifier,
            format!("car={} laps={}", p.payload.car_idx(), p.payload.num_laps),
        ),
        V2Packet::TyreSets(p) => (
            "TyreSets",
            p.header.frame_identifier,
            format!("car={} fitted={}", p.payload.car_idx(), p.payload.fitted_idx()),
        ),
        V2Packet::CarMotionEx(p) => ("CarMotionEx", p.header.frame_identifier, String::new()),
        V2Packet::TimeTrial(p) => {
            let best_lap = p.payload.player_session_best.lap_time_in_ms;
            ("TimeTrial", p.header.frame_identifier, format!("best_lap={best_lap}ms"))
        }
        V2Packet::LapPositions(p) => ("LapPositions", p.header.frame_identifier, String::new()),
    }
}

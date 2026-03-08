//! Round-trip tests using real packet bytes captured by `examples/v2_monitor`.
//!
//! Each test loads `test_packets/<Type>.bin` (if present) and asserts that
//! `v2::parse::parse` succeeds and returns the expected variant.
//!
//! Run the monitor first to populate the fixture files:
//!   cargo run --example v2_monitor

use f1_game_library_models_25::parse::{self, V2Packet};

fn load(name: &str) -> Option<Vec<u8>> {
    let path = format!("test_packets/{name}.bin");
    std::fs::read(&path).ok()
}

macro_rules! fixture_test {
    ($test_name:ident, $file:literal, $variant:ident) => {
        #[test]
        fn $test_name() {
            let Some(bytes) = load($file) else {
                eprintln!("skipping {}: fixture not found (run v2_monitor first)", $file);
                return;
            };
            match parse::parse(&bytes) {
                Ok(V2Packet::$variant(_)) => {}
                Ok(other) => panic!("expected {} variant, got {:?}", stringify!($variant), other),
                Err(e) => panic!("parse failed: {e}"),
            }
        }
    };
}

fixture_test!(motion, "Motion", Motion);
fixture_test!(session, "Session", Session);
fixture_test!(lap_data, "LapData", LapData);
fixture_test!(event, "Event", Event);
fixture_test!(participants, "Participants", Participants);
fixture_test!(car_setups, "CarSetups", CarSetups);
fixture_test!(car_telemetry, "CarTelemetry", CarTelemetry);
fixture_test!(car_status, "CarStatus", CarStatus);
fixture_test!(final_classification, "FinalClassification", FinalClassification);
fixture_test!(lobby_info, "LobbyInfo", LobbyInfo);
fixture_test!(car_damage, "CarDamage", CarDamage);
fixture_test!(session_history, "SessionHistory", SessionHistory);
fixture_test!(tyre_sets, "TyreSets", TyreSets);
fixture_test!(car_motion_ex, "CarMotionEx", CarMotionEx);
fixture_test!(time_trial, "TimeTrial", TimeTrial);
fixture_test!(lap_positions, "LapPositions", LapPositions);

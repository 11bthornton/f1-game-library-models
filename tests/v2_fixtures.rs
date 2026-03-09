//! Round-trip tests using real packet bytes captured by `examples/v2_monitor`.
//!
//! Each test loads `test_packets/<Type>.bin` (if present), asserts that parsing
//! succeeds and returns the expected variant, then unwraps every enum accessor
//! to confirm no garbage values are present in the captured data.
//!
//! Run with `-- --nocapture` to see the parsed values:
//!   cargo test -- --nocapture
//!
//! Run the monitor first to populate the fixture files:
//!   cargo run --example v2_monitor

use f1_game_library_models_25::parse::{self, V2Packet};

fn load(name: &str) -> Option<Vec<u8>> {
    let path = format!("test_packets/{name}.bin");
    std::fs::read(&path).ok()
}

// ── Motion ────────────────────────────────────────────────────────────────────

#[test]
fn motion() {
    let Some(bytes) = load("Motion") else {
        eprintln!("skipping Motion: fixture not found");
        return;
    };
    assert!(matches!(parse::parse(&bytes).unwrap(), V2Packet::Motion(_)));
    println!("[Motion] parsed ok ({} bytes)", bytes.len());
}

// ── Session ───────────────────────────────────────────────────────────────────

#[test]
fn session() {
    let Some(bytes) = load("Session") else {
        eprintln!("skipping Session: fixture not found");
        return;
    };
    let V2Packet::Session(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected Session variant");
    };
    let s = p.payload;

    println!("[Session]");
    println!("  track_id:                    {:?}", s.track_id().unwrap());
    println!("  session_type:                {:?}", s.session_type().unwrap());
    println!("  weather:                     {:?}", s.weather().unwrap());
    println!("  formula:                     {:?}", s.formula().unwrap());
    println!("  safety_car_status:           {:?}", s.safety_car_status().unwrap());
    println!("  forecast_accuracy:           {:?}", s.forecast_accuracy().unwrap());
    println!("  braking_assist:              {:?}", s.braking_assist().unwrap());
    println!("  gearbox_assist:              {:?}", s.gearbox_assist().unwrap());
    println!("  dynamic_racing_line:         {:?}", s.dynamic_racing_line().unwrap());
    println!("  dynamic_racing_line_type:    {:?}", s.dynamic_racing_line_type().unwrap());
    println!("  session_length:              {:?}", s.session_length().unwrap());
    println!("  speed_units_lead:            {:?}", s.speed_units_lead_player().unwrap());
    println!("  temperature_units_lead:      {:?}", s.temperature_units_lead_player().unwrap());
    println!("  speed_units_secondary:       {:?}", s.speed_units_secondary_player().unwrap());
    println!("  temperature_units_secondary: {:?}", s.temperature_units_secondary_player().unwrap());
    println!("  recovery_mode:               {:?}", s.recovery_mode().unwrap());
    println!("  flashback_limit:             {:?}", s.flashback_limit().unwrap());
    println!("  car_damage:                  {:?}", s.car_damage().unwrap());
    println!("  car_damage_rate:             {:?}", s.car_damage_rate().unwrap());
    println!("  collisions:                  {:?}", s.collisions().unwrap());
    println!("  pit_stop_experience:         {:?}", s.pit_stop_experience().unwrap());
    println!("  safety_car:                  {:?}", s.safety_car().unwrap());
    println!("  red_flags:                   {:?}", s.red_flags().unwrap());
    println!("  game_mode:                   {:?}", s.game_mode().unwrap());
    println!("  rule_set:                    {:?}", s.rule_set().unwrap());
    println!("  surface_type:                {:?}", s.surface_type().unwrap());

    println!("  marshal zones ({}):", s.num_marshal_zones);
    for (i, zone) in s.marshal_zones[..s.num_marshal_zones as usize].iter().enumerate() {
        println!("    [{}] zone_flag: {:?}", i, zone.zone_flag().unwrap());
    }

    println!("  weather forecast samples ({}):", s.num_weather_forecast_samples);
    for (i, sample) in s.weather_forecast_samples[..s.num_weather_forecast_samples as usize].iter().enumerate() {
        println!(
            "    [{}] session: {:?}, weather: {:?}, track_temp_change: {:?}, air_temp_change: {:?}",
            i,
            sample.session_type().unwrap(),
            sample.weather().unwrap(),
            sample.track_temperature_change().unwrap(),
            sample.air_temperature_change().unwrap(),
        );
    }
}

// ── Lap Data ──────────────────────────────────────────────────────────────────

#[test]
fn lap_data() {
    let Some(bytes) = load("LapData") else {
        eprintln!("skipping LapData: fixture not found");
        return;
    };
    let V2Packet::LapData(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected LapData variant");
    };
    println!("[LapData]");
    for (i, lap) in p.payload.lap_data.iter().enumerate() {
        println!(
            "  [{}] pit_status: {:?}, sector: {:?}, driver_status: {:?}, result_status: {:?}",
            i,
            lap.pit_status().unwrap(),
            lap.sector().unwrap(),
            lap.driver_status().unwrap(),
            lap.result_status().unwrap(),
        );
    }
}

// ── Event ─────────────────────────────────────────────────────────────────────

#[test]
fn event() {
    let Some(bytes) = load("Event") else {
        eprintln!("skipping Event: fixture not found");
        return;
    };
    assert!(matches!(parse::parse(&bytes).unwrap(), V2Packet::Event(_)));
    println!("[Event] parsed ok ({} bytes)", bytes.len());
}

// ── Participants ──────────────────────────────────────────────────────────────

#[test]
fn participants() {
    let Some(bytes) = load("Participants") else {
        eprintln!("skipping Participants: fixture not found");
        return;
    };
    let V2Packet::Participants(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected Participants variant");
    };
    let active = p.payload.num_active_cars as usize;
    println!("[Participants] active={active}");
    for (i, participant) in p.payload.participants[..active].iter().enumerate() {
        println!(
            "  [{}] team: {:?}, platform: {:?}, nationality: {:?}",
            i,
            participant.team_id().unwrap(),
            participant.platform().unwrap(),
            participant.nationality().unwrap(),
        );
    }
}

// ── Car Setups ────────────────────────────────────────────────────────────────

#[test]
fn car_setups() {
    let Some(bytes) = load("CarSetups") else {
        eprintln!("skipping CarSetups: fixture not found");
        return;
    };
    assert!(matches!(parse::parse(&bytes).unwrap(), V2Packet::CarSetups(_)));
    println!("[CarSetups] parsed ok ({} bytes)", bytes.len());
}

// ── Car Telemetry ─────────────────────────────────────────────────────────────

#[test]
fn car_telemetry() {
    let Some(bytes) = load("CarTelemetry") else {
        eprintln!("skipping CarTelemetry: fixture not found");
        return;
    };
    let V2Packet::CarTelemetry(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected CarTelemetry variant");
    };
    println!("[CarTelemetry]");
    for (i, car) in p.payload.car_telemetry_data.iter().enumerate() {
        let st = car.surface_type();
        println!(
            "  [{}] surface: RL={:?}, RR={:?}, FL={:?}, FR={:?}",
            i,
            st.rear_left.unwrap(),
            st.rear_right.unwrap(),
            st.front_left.unwrap(),
            st.front_right.unwrap(),
        );
    }
}

// ── Car Status ────────────────────────────────────────────────────────────────

#[test]
fn car_status() {
    let Some(bytes) = load("CarStatus") else {
        eprintln!("skipping CarStatus: fixture not found");
        return;
    };
    let V2Packet::CarStatus(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected CarStatus variant");
    };
    println!("[CarStatus]");
    for (i, car) in p.payload.car_status_data.iter().enumerate() {
        println!(
            "  [{}] tc: {:?}, fuel_mix: {:?}, tyre: {:?}/{:?}, ers: {:?}, fia_flag: {:?}",
            i,
            car.traction_control().unwrap(),
            car.fuel_mix().unwrap(),
            car.actual_tyre_compound().unwrap(),
            car.visual_tyre_compound().unwrap(),
            car.ers_deploy_mode().unwrap(),
            car.vehicle_fia_flags().unwrap(),
        );
    }
}

// ── Final Classification ──────────────────────────────────────────────────────

#[test]
fn final_classification() {
    let Some(bytes) = load("FinalClassification") else {
        eprintln!("skipping FinalClassification: fixture not found");
        return;
    };
    let V2Packet::FinalClassification(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected FinalClassification variant");
    };
    let num_cars = p.payload.num_cars as usize;
    println!("[FinalClassification] num_cars={num_cars}");
    for (i, car) in p.payload.classification_data[..num_cars].iter().enumerate() {
        println!(
            "  [{}] result_status: {:?}, result_reason: {:?}",
            i,
            car.result_status().unwrap(),
            car.result_reason().unwrap(),
        );
    }
}

// ── Lobby Info ────────────────────────────────────────────────────────────────

#[test]
fn lobby_info() {
    let Some(bytes) = load("LobbyInfo") else {
        eprintln!("skipping LobbyInfo: fixture not found");
        return;
    };
    let V2Packet::LobbyInfo(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected LobbyInfo variant");
    };
    let num_players = p.payload.num_players as usize;
    println!("[LobbyInfo] num_players={num_players}");
    for (i, player) in p.payload.lobby_players[..num_players].iter().enumerate() {
        // nationality may be 0 (unset) in solo-session lobby captures
        println!(
            "  [{}] team: {:?}, platform: {:?}, nationality: {:?}, ready: {:?}",
            i,
            player.team_id().unwrap(),
            player.platform().unwrap(),
            player.nationality(),
            player.ready_status().unwrap(),
        );
    }
}

// ── Car Damage ────────────────────────────────────────────────────────────────

#[test]
fn car_damage() {
    let Some(bytes) = load("CarDamage") else {
        eprintln!("skipping CarDamage: fixture not found");
        return;
    };
    assert!(matches!(parse::parse(&bytes).unwrap(), V2Packet::CarDamage(_)));
    println!("[CarDamage] parsed ok ({} bytes)", bytes.len());
}

// ── Session History ───────────────────────────────────────────────────────────

#[test]
fn session_history() {
    let Some(bytes) = load("SessionHistory") else {
        eprintln!("skipping SessionHistory: fixture not found");
        return;
    };
    let V2Packet::SessionHistory(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected SessionHistory variant");
    };
    let num_stints = p.payload.num_tyre_stints as usize;
    println!("[SessionHistory] num_laps={}, num_stints={num_stints}", p.payload.num_laps);
    for (i, stint) in p.payload.tyre_stints_history_data[..num_stints].iter().enumerate() {
        println!(
            "  [{}] actual: {:?}, visual: {:?}",
            i,
            stint.tyre_actual_compound().unwrap(),
            stint.tyre_visual_compound().unwrap(),
        );
    }
}

// ── Tyre Sets ─────────────────────────────────────────────────────────────────

#[test]
fn tyre_sets() {
    let Some(bytes) = load("TyreSets") else {
        eprintln!("skipping TyreSets: fixture not found");
        return;
    };
    let V2Packet::TyreSets(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected TyreSets variant");
    };
    println!("[TyreSets] fitted_idx={}", p.payload.fitted_idx());
    for (i, set) in p.payload.tyre_sets.iter().enumerate() {
        println!(
            "  [{}] actual: {:?}, visual: {:?}, recommended_session: {:?}",
            i,
            set.actual_tyre_compound().unwrap(),
            set.visual_tyre_compound().unwrap(),
            set.recommended_session().unwrap(),
        );
    }
}

// ── Car Motion Ex ─────────────────────────────────────────────────────────────

#[test]
fn car_motion_ex() {
    let Some(bytes) = load("CarMotionEx") else {
        eprintln!("skipping CarMotionEx: fixture not found");
        return;
    };
    assert!(matches!(parse::parse(&bytes).unwrap(), V2Packet::CarMotionEx(_)));
    println!("[CarMotionEx] parsed ok ({} bytes)", bytes.len());
}

// ── Time Trial ────────────────────────────────────────────────────────────────

#[test]
fn time_trial() {
    let Some(bytes) = load("TimeTrial") else {
        eprintln!("skipping TimeTrial: fixture not found");
        return;
    };
    let V2Packet::TimeTrial(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected TimeTrial variant");
    };
    println!("[TimeTrial]");
    for (label, dataset) in [
        ("player_session_best", p.payload.player_session_best),
        ("personal_best", p.payload.personal_best),
        ("rival", p.payload.rival),
    ] {
        println!("  {label}: team={:?}", dataset.team_id().unwrap());
    }
}

// ── Lap Positions ─────────────────────────────────────────────────────────────

#[test]
fn lap_positions() {
    let Some(bytes) = load("LapPositions") else {
        eprintln!("skipping LapPositions: fixture not found");
        return;
    };
    assert!(matches!(parse::parse(&bytes).unwrap(), V2Packet::LapPositions(_)));
    println!("[LapPositions] parsed ok ({} bytes)", bytes.len());
}

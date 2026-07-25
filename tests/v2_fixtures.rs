//! Round-trip tests using real packet bytes captured by `examples/v2_monitor`.
//!
//! Each test loads `test_packets/<Type>.bin` (if present), asserts that parsing
//! succeeds and returns the expected variant, then unwraps every enum accessor
//! to confirm no garbage values are present in the captured data.
//!
//! ## Correctness net
//!
//! Tests assert:
//! 1. Header `packet_id` matches the expected id for that file.
//! 2. Header `packet_format == 2025`.
//! 3. Field sanity bounds on the highest-signal packets (CarTelemetry, LapData,
//!    Motion) — these catch **offset / misalignment drift strongly** (misread
//!    bytes produce NaN floats, huge ints that blow past any sane range) but
//!    catch **same-size adjacent-field swaps weakly** (e.g. two adjacent f32s
//!    swapped would both still be in-range). This is a gross-error net, not
//!    proof of field-level correctness.
//!
//! Run with `-- --nocapture` to see the parsed values:
//!   cargo test -- --nocapture
//!
//! Run the monitor first to populate the fixture files:
//!   cargo run --example v2_monitor

use f1_game_library_models_25::packet_id::PacketId;
use f1_game_library_models_25::parse::{self, V2Packet};

fn load(name: &str) -> Option<Vec<u8>> {
    let path = format!("test_packets/{name}.bin");
    std::fs::read(&path).ok()
}

/// Assert common header invariants: packet_format is 2025 and packet_id matches.
fn assert_header(header: f1_game_library_models_25::PacketHeader, expected_id: PacketId) {
    assert_eq!(
        header.packet_format(),
        2025,
        "expected packet_format 2025, got {}",
        header.packet_format()
    );
    assert_eq!(
        header.packet_id().unwrap(),
        expected_id,
        "expected packet_id {expected_id:?}, got {:?}",
        header.packet_id()
    );
}

// ── Motion ────────────────────────────────────────────────────────────────────

#[test]
fn motion() {
    let Some(bytes) = load("Motion") else {
        eprintln!("skipping Motion: fixture not found");
        return;
    };
    let V2Packet::Motion(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected Motion variant");
    };

    assert_header(p.header, PacketId::Motion);

    // Sanity bounds: positional floats must be finite (not NaN/inf), g-force
    // within a plausible band. A misaligned read would produce garbage floats.
    let player = p.payload[p.header.player_car_index()];

    assert!(player.world_position_x().is_finite(), "position x not finite");
    assert!(player.world_position_y().is_finite(), "position y not finite");
    assert!(player.world_position_z().is_finite(), "position z not finite");
    assert!(player.world_velocity_x().is_finite(), "velocity x not finite");
    assert!(player.world_velocity_y().is_finite(), "velocity y not finite");
    assert!(player.world_velocity_z().is_finite(), "velocity z not finite");

    // G-force: even under extreme braking/cornering, values beyond ±10g are
    // implausible for an F1 car. A byte-offset error would typically produce
    // values in the thousands or NaN.
    let g_lat = player.g_force_lateral();
    let g_lon = player.g_force_longitudinal();
    let g_vert = player.g_force_vertical();
    assert!(
        g_lat.is_finite() && g_lat.abs() <= 10.0,
        "g_force_lateral out of range: {g_lat}"
    );
    assert!(
        g_lon.is_finite() && g_lon.abs() <= 10.0,
        "g_force_longitudinal out of range: {g_lon}"
    );
    assert!(
        g_vert.is_finite() && g_vert.abs() <= 10.0,
        "g_force_vertical out of range: {g_vert}"
    );

    assert!(player.yaw().is_finite(), "yaw not finite");
    assert!(player.pitch().is_finite(), "pitch not finite");
    assert!(player.roll().is_finite(), "roll not finite");

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

    assert_header(p.header, PacketId::Session);

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
    println!(
        "  dynamic_racing_line_type:    {:?}",
        s.dynamic_racing_line_type().unwrap()
    );
    println!("  session_length:              {:?}", s.session_length().unwrap());
    println!(
        "  speed_units_lead:            {:?}",
        s.speed_units_lead_player().unwrap()
    );
    println!(
        "  temperature_units_lead:      {:?}",
        s.temperature_units_lead_player().unwrap()
    );
    println!(
        "  speed_units_secondary:       {:?}",
        s.speed_units_secondary_player().unwrap()
    );
    println!(
        "  temperature_units_secondary: {:?}",
        s.temperature_units_secondary_player().unwrap()
    );
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
    for (i, sample) in s.weather_forecast_samples[..s.num_weather_forecast_samples as usize]
        .iter()
        .enumerate()
    {
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

    assert_header(p.header, PacketId::LapData);

    // Sanity bounds on each car's lap data.
    for (i, lap) in p.payload.lap_data.iter().enumerate() {
        // car_position is 0 for inactive cars, 1..=22 for active ones.
        assert!(
            lap.car_position <= 22,
            "car[{i}] car_position out of range: {}",
            lap.car_position
        );

        // lap_distance can be negative briefly (behind the start line), but
        // total_distance is cumulative and non-negative in a valid session.
        // A misaligned read would produce values like 1e38 or NaN.
        let total = lap.total_distance();
        assert!(total.is_finite(), "car[{i}] total_distance not finite: {total}");

        let lap_dist = lap.lap_distance();
        assert!(lap_dist.is_finite(), "car[{i}] lap_distance not finite: {lap_dist}");

        // Enum accessors must resolve for every car slot.
        lap.pit_status().unwrap();
        lap.sector().unwrap();
        lap.driver_status().unwrap();
        lap.result_status().unwrap();
    }

    println!("[LapData] parsed ok ({} bytes)", bytes.len());
}

// ── Event ─────────────────────────────────────────────────────────────────────

#[test]
fn event() {
    let Some(bytes) = load("Event") else {
        eprintln!("skipping Event: fixture not found");
        return;
    };
    let V2Packet::Event(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected Event variant");
    };
    assert_header(p.header, PacketId::Event);
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

    assert_header(p.header, PacketId::Participants);

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
    let V2Packet::CarSetups(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected CarSetups variant");
    };
    assert_header(p.header, PacketId::CarSetups);
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

    assert_header(p.header, PacketId::CarTelemetry);

    // Field sanity bounds across all 22 car slots. These ranges are tolerant of
    // garage/stationary frames (speed=0, gear=0 neutral). A byte-offset error
    // would typically produce values like speed=40000, throttle=1e20, or NaN.
    for (i, car) in p.payload.car_telemetry_data.iter().enumerate() {
        let speed = car.speed();
        assert!(speed <= 400, "car[{i}] speed out of range: {speed}");

        let gear = car.gear;
        assert!((-1..=8).contains(&gear), "car[{i}] gear out of range: {gear}");

        let throttle = car.throttle();
        assert!(
            throttle >= 0.0 && throttle <= 1.0,
            "car[{i}] throttle out of range: {throttle}"
        );

        let brake = car.brake();
        assert!(brake >= 0.0 && brake <= 1.0, "car[{i}] brake out of range: {brake}");

        let clutch = car.clutch;
        assert!(clutch <= 100, "car[{i}] clutch out of range: {clutch}");

        let rev = car.rev_lights_percent;
        assert!(rev <= 100, "car[{i}] rev_lights_percent out of range: {rev}");

        let rpm = car.engine_rpm();
        assert!(rpm < 20000, "car[{i}] engine_rpm out of range: {rpm}");

        // Surface type enum must resolve.
        let st = car.surface_type();
        st.rear_left.unwrap();
        st.rear_right.unwrap();
        st.front_left.unwrap();
        st.front_right.unwrap();
    }

    println!("[CarTelemetry] parsed ok ({} bytes)", bytes.len());
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

    assert_header(p.header, PacketId::CarStatus);

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

    assert_header(p.header, PacketId::FinalClassification);

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

    assert_header(p.header, PacketId::LobbyInfo);

    let num_players = p.payload.num_players as usize;
    println!("[LobbyInfo] num_players={num_players}");
    for (i, player) in p.payload.lobby_players[..num_players].iter().enumerate() {
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
    let V2Packet::CarDamage(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected CarDamage variant");
    };
    assert_header(p.header, PacketId::CarDamage);
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

    assert_header(p.header, PacketId::SessionHistory);

    let num_stints = p.payload.num_tyre_stints as usize;
    println!(
        "[SessionHistory] num_laps={}, num_stints={num_stints}",
        p.payload.num_laps
    );
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

    assert_header(p.header, PacketId::TyreSets);

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
    let V2Packet::CarMotionEx(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected CarMotionEx variant");
    };
    assert_header(p.header, PacketId::CarMotionEx);
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

    assert_header(p.header, PacketId::TimeTrial);

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
    let V2Packet::LapPositions(p) = parse::parse(&bytes).unwrap() else {
        panic!("expected LapPositions variant");
    };
    assert_header(p.header, PacketId::LapPositions);
    println!("[LapPositions] parsed ok ({} bytes)", bytes.len());
}

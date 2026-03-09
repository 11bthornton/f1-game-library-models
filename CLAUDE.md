# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`f1-game-library-models-25` is a Rust library that defines data types and deserialization logic for the UDP telemetry packets emitted by the EA Sports F1 2025 video game. It targets the [official UDP spec](https://forums.ea.com/blog/f1-games-game-info-hub-en/ea-sports%E2%84%A2-f1%C2%AE25-udp-specification/12187347). Previous game versions are not supported.

## Commands

```bash
# Build
cargo build

# Run all tests (preferred — matches CI)
cargo nextest run --workspace --all-features

# Run standard tests (if nextest is unavailable)
cargo test

# Run a single test by name
cargo nextest run <test_name>

# Lint
cargo clippy

# Format
cargo fmt

# Check for security advisories
cargo audit
```

CI uses `cargo nextest` on both Linux and Windows. Publishing to crates.io is triggered automatically on push to `main` via `cargo release`.

## Architecture

### Deserialization Pipeline

All incoming UDP data flows through a single public entry point:

```
deserialise_udp_packet_from_bytes(&[u8]) -> anyhow::Result<F1Data>
```

This function lives in `src/telemetry_data/mod.rs` and works as follows:
1. Reads the 29-byte `PacketHeader` from the front of the buffer using `PacketHeader::from_bytes`.
2. Dispatches on `PacketId` (extracted from the header) via the `deserialise_packet_type!` macro.
3. Each packet type is deserialized from the remaining bytes using the `FormulaOnePacket::from_bytes` blanket impl, which performs an unsafe `ptr::read_unaligned` cast.
4. Returns the appropriate `F1Data` enum variant.

Event packets (`PacketId::EventPacket`) are handled separately by `deserialise_event_packet_from_bytes` in `src/telemetry_data/event_data/packet.rs`.

### Key Traits

- **`FormulaOnePacket`** (`src/lib.rs`): Implemented by every raw packet struct. Requires `PACKET_SIZE`, `fix_endianness` (converts little-endian wire fields to native byte order), and provides the `from_bytes` default method.
- **`Pod`** (`src/lib.rs`, unsafe): Marker trait that asserts a type is safe to construct via raw byte reinterpretation. Must be implemented (`unsafe impl`) for every struct that is deserialized from raw bytes.

### Module Layout

`src/telemetry_data/` contains one sub-module per packet type, each following the same pattern:
- A `packet.rs` (or similar) defines the top-level array/container struct that implements `FormulaOnePacket` + `Pod`.
- Supporting files define the per-car data structs and domain enums (e.g., `CarDamageData`, `WingDamage`, `EngineWear`).
- A `mod.rs` re-exports the public types.

All packet modules are re-exported flat from `src/telemetry_data/mod.rs` so consumers only need `use f1_game_library_models_25::telemetry_data::*`.

### Shared Utilities

- **`src/telemetry_data/utility/wheel_data.rs`**: Generic `WheelData<T>` struct used throughout for per-wheel values (rear_left, rear_right, front_left, front_right).
- **`src/utils/mod.rs`**: `u8_as_usize` serde helper for fields that are `u8` on the wire but need to be `usize` for array indexing.
- **`src/lib.rs`**: Exports `constants` (packet sizes, `MAX_CARS_IN_SESSION = 22`), the `FormulaOnePacket` trait, and the top-level `deserialise_udp_packet_from_bytes` function.

### Adding a New Packet Type

1. Create a new module directory under `src/telemetry_data/<type_name>/`.
2. Define the per-car data struct and any supporting enums.
3. Define the array/container struct, implement `Pod` (unsafe) and `FormulaOnePacket` with the correct `PACKET_SIZE` constant from `src/lib.rs::constants`.
4. Add the module to `src/telemetry_data/mod.rs` and re-export it.
5. Add a variant to `F1Data` and a corresponding arm in the `deserialise_packet_type!` macro call in `deserialise_udp_packet_from_bytes`.

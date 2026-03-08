//! UDP telemetry client for the v2 packet model.
//!
//! Binds a UDP socket, receives raw datagrams, parses them with [`crate::parse`],
//! and dispatches each [`V2Packet`] variant to the appropriate method on a user-supplied
//! [`HandlePacket`] implementation.
//!
//! Requires the `client` feature (pulls in `tokio`).
//!
//! # Example
//! ```no_run
//! use f1_game_library_models_25::v2::client::{TelemetryClient, HandlePacket, TelemetryControl};
//! use f1_game_library_models_25::v2::packets::lap_data::PacketLapData;
//!
//! struct MyHandler;
//!
//! impl HandlePacket for MyHandler {
//!     async fn handle_lap_data(&mut self, packet: PacketLapData) -> anyhow::Result<TelemetryControl> {
//!         println!("lap data: {:?}", packet.header.session_uid);
//!         Ok(TelemetryControl::Continue)
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut client = TelemetryClient::new("0.0.0.0:20777").await?;
//!     client.listen(&mut MyHandler).await
//! }
//! ```

use tokio::net::{ToSocketAddrs, UdpSocket};

use super::packets::{
    car_damage::PacketCarDamage, car_setups::PacketCarSetups, car_status::PacketCarStatus,
    car_telemetry::PacketCarTelemetry, event::PacketEvent, final_classification::PacketFinalClassification,
    lap_data::PacketLapData, lap_positions::PacketLapPositions, lobby_info::PacketLobbyInfo, motion::PacketMotion,
    motion_ex::PacketMotionEx, participants::PacketParticipants, session::PacketSession,
    session_history::PacketSessionHistory, time_trial::PacketTimeTrial, tyre_sets::PacketTyreSets,
};
use super::parse::{self, V2Packet};

/// Maximum size of any v2 packet (SessionHistory = 1460 bytes; 2048 gives headroom).
const BUFFER_SIZE: usize = 2048;

/// Controls whether the client keeps running after handling a packet.
pub enum TelemetryControl {
    /// Keep listening for more packets.
    Continue,
    /// Stop the listen loop and return from [`TelemetryClient::listen`].
    Stop,
}

/// UDP listener that parses v2 packets and dispatches them to a [`HandlePacket`] impl.
pub struct TelemetryClient {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
}

impl TelemetryClient {
    /// Bind to `addr` and return a ready-to-use client.
    pub async fn new<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr).await?,
            buf: [0; BUFFER_SIZE],
        })
    }

    /// Receive packets in a loop, dispatching each one to `handler`.
    ///
    /// Returns when the handler returns [`TelemetryControl::Stop`] or a fatal
    /// error occurs.
    pub async fn listen<H: HandlePacket>(&mut self, handler: &mut H) -> anyhow::Result<()> {
        loop {
            let (len, _) = self.socket.recv_from(&mut self.buf).await?;

            let packet = match parse::parse(&self.buf[..len]) {
                Ok(p) => p,
                Err(e) => {
                    // Unknown or malformed packet — log and continue rather than
                    // crashing the whole session.
                    eprintln!("[v2 client] parse error: {e}");
                    continue;
                }
            };

            let ctrl = dispatch(handler, packet).await?;
            if matches!(ctrl, TelemetryControl::Stop) {
                return Ok(());
            }
        }
    }
}

async fn dispatch<H: HandlePacket>(handler: &mut H, packet: V2Packet) -> anyhow::Result<TelemetryControl> {
    match packet {
        V2Packet::Motion(p) => handler.handle_motion(p).await,
        V2Packet::CarDamage(p) => handler.handle_car_damage(p).await,
        V2Packet::CarMotionEx(p) => handler.handle_car_motion_ex(p).await,
        V2Packet::TimeTrial(p) => handler.handle_time_trial(p).await,
        V2Packet::TyreSets(p) => handler.handle_tyre_sets(p).await,
        V2Packet::CarSetups(p) => handler.handle_car_setups(p).await,
        V2Packet::LapData(p) => handler.handle_lap_data(p).await,
        V2Packet::LapPositions(p) => handler.handle_lap_positions(p).await,
        V2Packet::SessionHistory(p) => handler.handle_session_history(p).await,
        V2Packet::CarTelemetry(p) => handler.handle_car_telemetry(p).await,
        V2Packet::LobbyInfo(p) => handler.handle_lobby_info(p).await,
        V2Packet::Participants(p) => handler.handle_participants(p).await,
        V2Packet::FinalClassification(p) => handler.handle_final_classification(p).await,
        V2Packet::CarStatus(p) => handler.handle_car_status(p).await,
        V2Packet::Session(p) => handler.handle_session(p).await,
        V2Packet::Event(p) => handler.handle_event(p).await,
    }
}

/// Implement this trait to handle v2 telemetry packets.
///
/// Every method has a default no-op implementation that returns
/// [`TelemetryControl::Continue`], so you only need to override the packets
/// you care about.
pub trait HandlePacket {
    fn handle_motion(&mut self, _p: PacketMotion) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_car_damage(&mut self, _p: PacketCarDamage) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_car_motion_ex(&mut self, _p: PacketMotionEx) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_time_trial(&mut self, _p: PacketTimeTrial) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_tyre_sets(&mut self, _p: PacketTyreSets) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_car_setups(&mut self, _p: PacketCarSetups) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_lap_data(&mut self, _p: PacketLapData) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_lap_positions(
        &mut self,
        _p: PacketLapPositions,
    ) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_session_history(
        &mut self,
        _p: PacketSessionHistory,
    ) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_car_telemetry(
        &mut self,
        _p: PacketCarTelemetry,
    ) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_lobby_info(&mut self, _p: PacketLobbyInfo) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_participants(
        &mut self,
        _p: PacketParticipants,
    ) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_final_classification(
        &mut self,
        _p: PacketFinalClassification,
    ) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_car_status(&mut self, _p: PacketCarStatus) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_session(&mut self, _p: PacketSession) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
    fn handle_event(&mut self, _p: PacketEvent) -> impl Future<Output = anyhow::Result<TelemetryControl>> {
        async { Ok(TelemetryControl::Continue) }
    }
}

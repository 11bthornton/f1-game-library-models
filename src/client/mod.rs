//! Telemetry client for receiving and handling F1 telemetry data over UDP.
//! This module provides a `TelemetryClient` that listens for UDP packets
//! from the F1 game and dispatches them to user-defined handlers based on packet type.
//! It supports asynchronous operation using Tokio.
//!
//! The client can be configured to listen on a specified address and port,
//! and users can implement the `HandlePacket` trait to define custom behavior
//! for each type of telemetry data received.
//!
//! This module needs to be enabled with the `client` feature flag.
//!
//! # Example Usage
//! ```no_run
//! use f1_game_library_models_25::client::{TelemetryClient, HandlePacket, TelemetryControl};
//! use f1_game_library_models_25::telemetry_data::PacketLapData;
//!
//! struct MyHandler;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Implement your packet handler
//!     struct MyHandler;
//!
//!     impl HandlePacket for MyHandler {
//!         async fn handle_lap_data(&mut self, data: PacketLapData) -> anyhow::Result<TelemetryControl> {
//!             println!("Received lap data: {:?}", data);
//!             Ok(TelemetryControl::Continue)
//!         }
//!         // Implement other methods...
//!     }
//!
//!     // Instantiate the handler
//!     let mut handler = MyHandler;
//!
//!     // Create the telemetry client, binding to the desired address and port
//!     let mut client = TelemetryClient::new_from_address("0.0.0.0:20777").await?;
//!
//!     // Start listening for packets
//!     client.listen(&mut handler).await?;
//!
//!     Ok(())
//! }
//! ```

use tokio::net::ToSocketAddrs;
use tokio::net::UdpSocket;

const BUFFER_SIZE: usize = 2048;

/// A client for receiving and handling F1 telemetry data over UDP.
///
/// `TelemetryClient` manages a UDP socket connection to listen for incoming telemetry packets
/// from the F1 game. It provides asynchronous methods to receive, buffer, and dispatch packets
/// to user-defined handlers implementing the [`HandlePacket`] trait.
///
/// The client is typically created using [`TelemetryClient::new_from_address`], specifying the
/// address and port to bind to. Once created, call [`TelemetryClient::listen`] to start
/// processing incoming packets.
///
/// # Example
/// See the module-level documentation for a usage example.
pub struct TelemetryClient {
    /// The underlying UDP socket used to receive telemetry packets.
    _udp_listener: UdpSocket,
    /// Buffer for storing incoming UDP packet data.
    _buffer: [u8; BUFFER_SIZE],
}

impl TelemetryClient {
    pub async fn new_from_address<A>(addr: A) -> anyhow::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            _udp_listener: tokio::net::UdpSocket::bind(addr).await?,
            _buffer: [0; BUFFER_SIZE],
        })
    }

    pub async fn listen<H>(&mut self, handler: &mut H) -> anyhow::Result<()>
    where
        H: HandlePacket,
    {
        loop {
            let (len, _) = self._udp_listener.recv_from(&mut self._buffer).await?;
            
            let packet = crate::deserialise_udp_packet_from_bytes(&self._buffer[..len])?;

            if len == 753 {
                println!("Received packet with len {len} {:#?}", packet);
            }

            match self._dispatch_packet(handler, packet).await? {
                TelemetryControl::Continue => continue,
                TelemetryControl::Stop => break Ok(()),
            }
        }
    }
}

/// Control flow for telemetry packet processing.
///
/// This enum is returned from packet handler methods to indicate whether the
/// `TelemetryClient` should continue processing incoming packets or stop listening.
///
/// # When to use each variant
/// - [`Continue`]: Return this to keep the client listening for more packets.
/// - [`Stop`]: Return this to stop the client and break out of the listening loop.
pub enum TelemetryControl {
    /// Continue processing and listening for more telemetry packets.
    ///
    /// Return this from a packet handler when you want the client to keep running.
    Continue,
    /// Stop processing and exit the telemetry listening loop.
    ///
    /// Return this from a packet handler when you want to terminate the client.
    Stop,
}

macro_rules! define_packet_handlers {
    (
        pub trait $trait_name:ident {
            $(
                $fn_name:ident ( $data_ty:path )
            ),+ $(,)?
        }

        enum $enum_ty:path {
            $(
                $variant:ident => $fn_map:ident
            ),+ $(,)?
        }
    ) => {
        /// # Example usage:
        /// ```no_run
        /// use f1_game_library_models_25::client::{TelemetryClient, HandlePacket, TelemetryControl};
        /// use f1_game_library_models_25::telemetry_data::PacketLapData;
        ///
        /// struct MyHandler;
        /// impl HandlePacket for MyHandler {
        ///    async fn handle_lap_data(&mut self, data: PacketLapData) -> anyhow::Result<TelemetryControl> {
        ///        println!("Received lap data: {:?}", data);
        ///        Ok(TelemetryControl::Continue)
        ///    }
        /// }
        /// ```
        pub trait $trait_name {
            $(
                #[allow(unused_variables)]
                fn $fn_name(&mut self, data: $data_ty) -> impl Future<Output = anyhow::Result<TelemetryControl>>
                {
                    // Default implementation: continue processing
                    async { Ok(TelemetryControl::Continue) }
                }
            )+
        }

        impl TelemetryClient {
            async fn _dispatch_packet<H>(
                &mut self,
                handler: &mut H,
                packet: $enum_ty,
            ) -> anyhow::Result<TelemetryControl>
            where
                H: $trait_name,
            {
                use $enum_ty::*;

                match packet {
                    $(
                        $variant(data) => handler.$fn_map(data).await,
                    )+
                    None => Ok(TelemetryControl::Continue),
                }
            }
        }
    };
}

define_packet_handlers! {
    pub trait HandlePacket {
        handle_lap_data(crate::telemetry_data::PacketLapData),
        handle_car_damage_data(crate::telemetry_data::PacketCarDamageData),
        handle_car_setup_data(crate::telemetry_data::PacketCarSetupData),
        handle_car_status_data(crate::telemetry_data::PacketCarStatusData),
        handle_car_motion_data(crate::telemetry_data::PacketCarMotionData),
        handle_participant_data(crate::telemetry_data::PacketParticipantData),
        handle_session_data(crate::telemetry_data::PacketSessionData),
        handle_event_data(crate::telemetry_data::PacketEventData),
        handle_telemetry_data(crate::telemetry_data::PacketCarTelemetryData),
        handle_classification_data(crate::telemetry_data::PacketClassificationData),
        handle_session_history_data(crate::telemetry_data::PacketSessionHistoryData),
        handle_lobby_data(crate::telemetry_data::PacketLobbyInfoData),
        handle_extended_motion_data(crate::telemetry_data::PacketCarMotionExData),
        handle_tyre_set_data(crate::telemetry_data::PacketTyreSetsData),
        handle_time_trial_data(crate::telemetry_data::PacketTimeTrialData),
        handle_lap_positions_data(crate::telemetry_data::PacketLapPositionsData),
    }

    enum crate::telemetry_data::F1Data {
        LapData => handle_lap_data,
        CarDamageData => handle_car_damage_data,
        CarSetupData => handle_car_setup_data,
        CarStatusData => handle_car_status_data,
        CarMotionData => handle_car_motion_data,
        ParticipantData => handle_participant_data,
        SessionData => handle_session_data,
        EventData => handle_event_data,
        TelemetryData => handle_telemetry_data,
        ClassificationData => handle_classification_data,
        SessionHistoryData => handle_session_history_data,
        LobbyData => handle_lobby_data,
        ExtendedMotionData => handle_extended_motion_data,
        TyreSetData => handle_tyre_set_data,
        TimeTrialData => handle_time_trial_data,
        LapPositionsData => handle_lap_positions_data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::net::UdpSocket;
    use tokio::sync::Mutex;

    struct TestHandler {
        called: Arc<Mutex<bool>>,
    }

    impl HandlePacket for TestHandler {
        async fn handle_lap_data(
            &mut self,
            _data: crate::telemetry_data::PacketLapData,
        ) -> anyhow::Result<TelemetryControl> {
            let mut called = self.called.lock().await;
            *called = true;
            Ok(TelemetryControl::Stop)
        }
    }

    trait DummyBytes {
        fn to_bytes() -> Vec<u8>;
    }

    impl DummyBytes for crate::telemetry_data::PacketLapData {
        fn to_bytes() -> Vec<u8> {
            let packet = crate::telemetry_data::PacketLapData::default();
            bincode::serialize(&packet).unwrap()
        }
    }

    #[tokio::test]
    async fn telemetry_client_receives_packet() {
        let addr = "127.0.0.1:54321";
        let mut client = TelemetryClient::new_from_address(addr).await.unwrap();

        let called_flag = Arc::new(Mutex::new(false));
        let mut handler = TestHandler {
            called: called_flag.clone(),
        };

        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let packet_bytes = crate::telemetry_data::PacketLapData::to_bytes();
        socket.send_to(&packet_bytes, addr).await.unwrap();

        client.listen(&mut handler).await.unwrap();

        assert!(*called_flag.lock().await);
    }
}

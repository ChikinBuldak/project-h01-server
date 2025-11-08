use crate::network::{
    system::handle_client_message,
    types::{ClientMessage, ClientMessageReceiver, OutboundMessage, RoomConfig},
};
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use crossbeam_channel;
use std::time::Duration;

pub mod types {
    use bevy::{ecs::resource::Resource, platform::collections::HashMap};
    use serde::{Deserialize, Serialize};
    use webrtc::peer_connection::RTCPeerConnection;

    #[derive(Debug, Deserialize)]
    pub enum ClientAction {
        EntityState,
        InitializeRoom {
            /// The list of player UUIDs that will be joining.
            player_ids: Vec<String>,
        },
    }

    /// Valid client messages received by the server
    /// NOTE: This might changed since I need some readjustment with the client data
    #[derive(Debug, Deserialize)]
    pub struct ClientMessage {
        pub player_id: String, // an uuid
        pub room_id: String,   // an uuid
        pub action: ClientAction,
    }

    /// Messages the Bevy Server sends *OUT* to the signaling server.
    #[derive(Debug, Serialize)]
    pub enum ServerSignal {
        /// The server sending an Offer to a specific player.
        SendOffer { sdp: String },
        /// The server sending an ICE candidate to a specific player.
        SendIceCandidate { candidate: String },
        // You would also have variants for game state, etc.
    }

    /// The wrapper for all outbound messages.
    #[derive(Debug, Serialize)]
    pub struct OutboundMessage {
        /// The room this message is from.
        pub room_id: String,
        /// The specific player this message is intended for.
        pub target_player_id: String,
        /// The actual signal payload.
        pub signal: ServerSignal,
    }

    #[derive(Resource)]
    pub struct RoomConfig {
        pub room_id: String,
    }

    /// Resource wrapper for the message receiver
    #[derive(Resource)]
    pub struct ClientMessageReceiver(pub crossbeam_channel::Receiver<ClientMessage>);

    #[derive(Resource)]
    pub struct OutboundMessageSender(pub crossbeam_channel::Sender<OutboundMessage>);

    #[derive(Resource, Default)]
    pub struct NetworkState {
        pub peer_connections: HashMap<String, RTCPeerConnection>,
        pub room_id: String,
    }
}

pub mod system {
    use crate::network::types::{
        ClientAction, ClientMessageReceiver, NetworkState, OutboundMessageSender, RoomConfig,
    };
    use bevy::prelude::*;
    use webrtc::peer_connection::RTCPeerConnection;

    pub fn handle_client_message(
        mut state: ResMut<NetworkState>,
        rx: Res<ClientMessageReceiver>,
        tx: Res<OutboundMessageSender>,
    ) {
        let receiver = &rx.0;
        let sender = &tx.0;

        while let Ok(message) = receiver.try_recv() {
            match message.action {
                ClientAction::InitializeRoom { player_ids } => {
                    info!(
                        "Received InitializeRoom command for room {}. Creating peers for: {:?}",
                        message.room_id, player_ids
                    );

                    state.room_id = message.room_id.clone();
                    // For each player, create a connection, create an offer,
                    // store the connection, and send the offer.
                    for player_id in player_ids {
                        info!("Setting up peer: {}", player_id);

                        let mut peer_conn = RTCPeerConnection::new()
                    }
                }
                ClientAction::EntityState => {}
            }
        }
    }
}

pub fn run_room(
    room_id: String,
    rx: crossbeam_channel::Receiver<ClientMessage>,
    tx: crossbeam_channel::Sender<OutboundMessage>,
) {
    let mut app = App::new();
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
    );

    app.insert_resource(RoomConfig { room_id });
    app.insert_resource(ClientMessageReceiver(rx));

    app.add_systems(Update, handle_client_message);

    app.run();
}

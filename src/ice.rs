use webrtc::api::APIBuilder;
use webrtc::peer_connection::{RTCPeerConnection, RTCConfiguration, ice_server::RTCIceServer};
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::media_engine::MediaEngine;
use webrtc::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the tokio executor
    let mut media_engine = MediaEngine::default();
    let api = APIBuilder::new().with_media_engine(media_engine).build();

    // Create a new RTCPeerConnection
    let mut config = RTCConfiguration::default();
    
    // Setup ICE servers (This example uses a public STUN server)
    config.ice_servers = vec![RTCIceServer {
        urls: vec!["stun:stun.l.google.com:19302".to_string()],
        ..Default::default()
    }];

    let peer_connection = Arc::new(Mutex::new(api.new_peer_connection(config).await?));

    // Handle ICE candidates (this example just prints them)
    let pc_clone = Arc::clone(&peer_connection);
    peer_connection.lock().await.on_ice_candidate(Box::new(move |candidate: Option<String>| {
        if let Some(c) = candidate {
            println!("New ICE candidate: {:?}", c);
        }
        Box::pin(async {})
    })).await;

    // Handle connection state changes
    let pc_clone_state = Arc::clone(&peer_connection);
    peer_connection.lock().await.on_peer_connection_state_change(Box::new(move |state: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {:?}", state);

        if state == RTCPeerConnectionState::Connected {
            println!("Connection established!");
        }

        Box::pin(async {
            let pc = pc_clone_state.lock().await;
            // You can add additional handling here based on the state
        })
    })).await;

    // Keep the connection open - simulate long running operations
    println!("Waiting for connections...");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

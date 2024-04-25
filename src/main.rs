use webrtc::api::APIBuilder;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::util::Conn;

pub mod ice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the tokio executor
    let api = APIBuilder::new().build();

    // Create a new RTCPeerConnection
    let config = RTCConfiguration::default();
    let peer_connection = api.new_peer_connection(config).await?;

    // Handle ICE candidates (this example just prints them)
    // move handes lifetimes to RTCPeerConnection
    let on_ice_candidate = Box::new(move |candidate: Option<String>| {
        if let Some(c) = candidate {
        println!("New ICE candidate: {:?}", c);
        }
        Box::pin(async {})
        });
        
        peer_connection.on_ice_candidate(on_ice_candidate).await;
        
        // Create an offer to establish a connection
        let offer = peer_connection.create_offer(None).await?;
        println!("Created offer: {:?}", offer);
        
        // Set the local description to the generated offer
        peer_connection.set_local_description(offer).await?;
        
        // Here you would typically send the offer to the remote peer through your signaling channel
        // For example, using a WebSocket or another HTTP-based mechanism
        
        // In a complete application, you would also need to handle the answer from the remote peer:
        // 1. Receive the answer via signaling
        // 2. Set the remote description with the received answer
        
        // Subscribe to connection state changes
        let on_connection_state_change = Box::new(move |state: RTCPeerConnectionState| {
        println!("Connection state changed: {:?}", state);
        Box::pin(async {})
        });
        peer_connection.on_connection_state_change(on_connection_state_change).await;
        
        // Keep the tokio executor alive.
        // In a real application, you might wait for a specific event or condition before exiting.
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("Shutting down...");
        
        Ok(())
    }

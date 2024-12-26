use tokio;
use tokio_tungstenite::accept_async;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::WebSocketStream;
use futures::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::error::Error;
use std::collections::HashMap;
use tokio::sync::Mutex;

// Add new message types
#[derive(Debug, Serialize, Deserialize)]
pub enum WebSocketMessage {
    InventoryUpdate(InventoryUpdate),
    GetSellers { request_id: String },
    SellersResponse { request_id: String, sellers: Vec<Seller> },
    Broadcast { message: String },
}

pub struct WebSocketServer {
    inventory_manager: Arc<InventoryManager>,
    notification_manager: Arc<NotificationManager>,
    broadcast_tx: broadcast::Sender<String>,
    // Add connected clients map
    connected_clients: Arc<Mutex<HashMap<String, WebSocketSender>>>,
}

type WebSocketSender = futures::stream::SplitSink<WebSocketStream<TcpStream>, tungstenite::Message>;

impl WebSocketServer {
    pub async fn new(
        inventory_manager: Arc<InventoryManager>,
        notification_manager: Arc<NotificationManager>
    ) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            inventory_manager,
            notification_manager,
            broadcast_tx: tx,
            connected_clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("WebSocket server listening on ws://127.0.0.1:8080");

        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream).await?;
            let tx = self.broadcast_tx.clone();
            let inventory_manager = self.inventory_manager.clone();
            let notification_manager = self.notification_manager.clone();
            let connected_clients = self.connected_clients.clone();

            tokio::spawn(async move {
                Self::handle_connection(ws_stream, tx, inventory_manager, notification_manager, connected_clients).await;
            });
        }
        Ok(())
    }

    async fn handle_connection(
        ws_stream: WebSocketStream<TcpStream>,
        tx: broadcast::Sender<String>,
        inventory_manager: Arc<InventoryManager>,
        notification_manager: Arc<NotificationManager>,
        connected_clients: Arc<Mutex<HashMap<String, WebSocketSender>>>,
    ) {
        let (ws_sender, mut ws_receiver) = ws_stream.split();
        let mut rx = tx.subscribe();

        // Store the sender in the connected_clients map
        let client_id = uuid::Uuid::new_v4().to_string();
        connected_clients.lock().await.insert(client_id.clone(), ws_sender);

        // Handle incoming messages
        while let Some(msg) = ws_receiver.next().await {
            if let Ok(msg) = msg {
                if let Ok(message) = serde_json::from_str::<WebSocketMessage>(&msg.to_string()) {
                    match message {
                        WebSocketMessage::InventoryUpdate(update) => {
                            inventory_manager.handle_update(update).await;
                        },
                        WebSocketMessage::GetSellers { request_id } => {
                            // Get all sellers and broadcast a message to them
                            if let Ok(sellers) = notification_manager.get_all_sellers().await {
                                let broadcast_msg = WebSocketMessage::Broadcast {
                                    message: "Test broadcast message".to_string(),
                                };
                                
                                if let Ok(msg_str) = serde_json::to_string(&broadcast_msg) {
                                    let clients = connected_clients.lock().await;
                                    for (_, sender) in clients.iter() {
                                        if let Err(e) = sender.send(msg_str.clone().into()).await {
                                            log::error!("Failed to send broadcast: {}", e);
                                        }
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        // Remove client when disconnected
        connected_clients.lock().await.remove(&client_id);
    }

    // New method to broadcast to all connected clients
    pub async fn broadcast_to_all(&self, message: String) -> Result<(), Box<dyn Error>> {
        let clients = self.connected_clients.lock().await;
        for (_, sender) in clients.iter() {
            if let Err(e) = sender.send(message.clone().into()).await {
                log::error!("Failed to broadcast message: {}", e);
            }
        }
        Ok(())
    }
}
use std::collections::HashMap;
use std::ffi::OsString;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;
use futures_channel::mpsc::UnboundedSender;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio_tungstenite::tungstenite::Message;
use tokio_websockets::WebSocketStream;
use uuid::Uuid;
// use tokio_tungstenite::tungstenite::protocol::Message;


use crate::game_repository::GameRepository;
use crate::connection_manager::ConnectionManager;
use crate::game::Game;
use crate::game_manager::GameManager;
use crate::response::Response;



type Tx = UnboundedSender<Message>;


#[derive(Debug, Clone)]
pub enum Event {
    GameCreated { game_id: Uuid },
    MoveMade {
        game_id: Uuid,
        message: String,
        columns:String,
        rows: String,
        board: HashMap<String, (String, Vec<String>)>,
    },
    Default {},
}


impl From<Response> for Event {
    fn from(value: Response) -> Self {
        match value {
            Response::MakeMoveResponse { game_id, message, columns, rows, board} =>
                Event::MoveMade { game_id, message, columns, rows, board },
            _ => Event::Default {},
        }
    }
}


pub struct EventService {
    sender: broadcast::Sender<Response>,
    game_manager: Arc<RwLock<GameManager>>,
}


impl EventService {
    // Create a new EventService with a specified buffer size
    pub fn new(buffer_size: usize, game_manager: Arc<RwLock<GameManager>>) -> Self {
        let (sender, _) = broadcast::channel(buffer_size);
        EventService { sender, game_manager }
    }

    // Publish an event
    pub async fn publish(&self, response: &Response) {
        match response {
            Response::CreateGameResponse { game_id, message } => {
                println!("Game created with ID: {}.\n{}", game_id, message);
            },

            Response::AuthorizeWebsocketConnectionResponse { game_id, user_id, connection_id, message } => {
                println!("{}", message);
                self.send_authorized_message(game_id.clone(), user_id.clone(), connection_id.clone(), message.clone()).await;
            },

            Response::MakeMoveResponse { game_id, message, columns, rows, board } => {
                self.send_move_made_message(game_id.clone(), message.clone(), columns.clone(), rows.clone(), board.clone()).await
            },
            _ => {},
        }
        // if let Err(e) = self.sender.send(event) {
        //     eprintln!("Failed to send event: {}", e);
        // }
    }

    // Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Response> {
        self.sender.subscribe()
    }

    async fn send_authorized_message(
        &self,
        game_id: Uuid,
        user_id: String,
        connection_id: SocketAddr,
        message: String) {

        // let peers = peer_map.lock().await;
        //
        // // We want to broadcast the message to everyone except ourselves.
        // let broadcast_recipients =
        //     peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
        //
        // for recp in broadcast_recipients {
        //     recp.unbounded_send(msg.clone()).unwrap();
        // }

        let game_manager_lock = self.game_manager.read().await;
        match game_manager_lock.connection_manager.ws_connection_id.get(&connection_id) {
            Some(connection) => {
                let response = Response::AuthorizeWebsocketConnectionResponse { game_id, user_id, connection_id, message, };
                let response_text = serde_json::to_string(&response).unwrap();

                // let message = tokio_websockets::Message::text(response_text.clone());

                let mut ws_connection = connection.value().lock().await;
                if let Err(e) = ws_connection.unbounded_send(Message::text(response_text.clone())) {
                    println!("Failed to send message to WebSocket connection: {}", e);
                } else {
                    println!("Message successfully sent to connection ID: {}", connection_id);
                }
                // connection.value().lock().await.send(message).await.unwrap();
            },
            _ => {},
        };
    }

    async fn send_move_made_message(
        &self,
        game_id: Uuid,
        message: String,
        columns: String,
        rows: String,
        board: HashMap<String, (String, Vec<String>)>,
    ) {
        let game_manager_lock = self.game_manager.read().await;
        match game_manager_lock.connection_manager.game_id_user_ids.get(&game_id) {
            Some(user_ids) => {
                let mut ws_connection_ids: Vec<SocketAddr> = Vec::new();
                for user_id in user_ids.clone() {
                    match game_manager_lock.connection_manager.user_id_ws_connection_ids.get(&user_id) {
                        Some(ids) => {
                            let ws_ids = ids.value();
                            ws_ids.iter().for_each(|item| ws_connection_ids.push(*item));
                        },
                        _ => {},
                    }
                }

                let mut connections: Vec<Arc<Mutex<Tx>>> = Vec::new();
                for id in ws_connection_ids {
                    match game_manager_lock.connection_manager.ws_connection_id.get(&id) {
                        Some(connection) => connections.push(connection.value().clone()),
                        _ => {},
                    }
                }
                let response = Response::MakeMoveResponse { game_id, message, columns, rows, board };
                // let response = serde_json::to_string(&response).unwrap();
                for mut connection in connections {
                    let mut ws_connection = connection.lock().await;
                    let response_text = serde_json::to_string(&response).unwrap();
                    let e = ws_connection.unbounded_send(Message::text(response_text.clone()));

                    // let message = tokio_websockets::Message::text(response.clone());
                    // connection.lock().await.send(message).await.unwrap();
                }
            },
            _ => {},
        };
    }
}
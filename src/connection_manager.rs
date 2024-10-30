use tokio::net::TcpStream;
use uuid::Uuid;
use std::{
    sync::Arc,
    net::SocketAddr
};

use tokio_websockets::WebSocketStream;
use tokio::sync::Mutex;
use dashmap::{DashMap, DashSet};
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

type Tx = UnboundedSender<Message>;

pub struct ConnectionManager {
    pub game_id_user_ids: Arc<DashMap<Uuid, DashSet<String>>>,
    pub user_id_game_ids: Arc<DashMap<String, DashSet<Uuid>>>,
    pub ws_connection_id: Arc<DashMap<SocketAddr, Arc<Mutex<Tx>>>>,
    pub user_id_ws_connection_ids: Arc<DashMap<String, DashSet<SocketAddr>>>,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            game_id_user_ids: Arc::new(DashMap::new()),
            user_id_game_ids: Arc::new(DashMap::new()),
            ws_connection_id: Arc::new(DashMap::new()),
            user_id_ws_connection_ids: Arc::new(DashMap::new()),
        }
    }

    pub fn add_connection(
        &mut self,
        game_id: &Uuid,
        user_id: &String,
        ws_connection_id: Option<SocketAddr>,
        ws_connection: Option<Arc<Mutex<Tx>>>,
    ) -> Result<String, String> {
        match (ws_connection_id, ws_connection) {
            (None, None) => {
                //create_game, join_game
                self.add_game_id(game_id, user_id);
                self.add_user_id(game_id, user_id);
                Ok("Connection added".to_string())
            },

            (Some(ws_connection_id), Some(ws_connection)) => {
                //authorize
                self.add_game_id(game_id, user_id);
                self.add_user_id(game_id, user_id);

                self.add_ws_connection(user_id, ws_connection_id, Arc::clone(&ws_connection));
                Ok("Connection added".to_string())
            },

            _ => {Err("Could not add connection".to_string())},
        }
    }

    fn add_game_id(&mut self, game_id: &Uuid, user_id: &String) {
        match self.game_id_user_ids.get(game_id) {
            Some(mut user_ids) => {
                user_ids.insert(user_id.clone());
            },
            None => {
                let user_ids = DashSet::new();
                user_ids.insert(user_id.to_string());
                self.game_id_user_ids.insert(*game_id, user_ids);
            },
        };
    }

    fn add_user_id(&mut self, game_id: &Uuid, user_id: &String) {
        match self.user_id_game_ids.get(user_id) {
            Some(mut game_ids) => {
                game_ids.insert(*game_id);
            },
            None => {
                let game_ids = DashSet::new();
                game_ids.insert(*game_id);
                self.user_id_game_ids.insert(user_id.clone(), game_ids);
            },
        };
    }

    fn add_ws_connection(
        &mut self,
        user_id: &String,
        ws_connection_id: SocketAddr,
        ws_connection: Arc<Mutex<Tx>>
    ) {
        match self.ws_connection_id.get(&ws_connection_id) {
            Some(_) => (),
            None => {
                self.ws_connection_id.insert(ws_connection_id, ws_connection);
            },
        };

        match self.user_id_ws_connection_ids.get(user_id) {
            Some(connections) => {
                connections.insert(ws_connection_id);
            },
            None => {
                let connection_ids = DashSet::new();
                connection_ids.insert(ws_connection_id);
                self.user_id_ws_connection_ids.insert(user_id.clone(), connection_ids);
            }
        }
    }
}
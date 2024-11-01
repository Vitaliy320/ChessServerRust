use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::Arc,
};
use std::ops::Deref;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::sync::{Mutex, RwLock};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::game_manager::GameManager;
use crate::request::{RequestEnum, AuthorizeWebsocketConnectionRequest, MakeMoveRequest};
use serde_json::from_str;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use crate::event_service::{Event, EventService};
use crate::request;
use crate::response::Response;
use crate::response::Response::RequestFailedResponse;
use crate::Board;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn run_websocket_server_new(game_manager: Arc<RwLock<GameManager>>) -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:8081".to_string());

    let event_service = Arc::new(RwLock::new(EventService::new(100, Arc::clone(&game_manager))));

    let mut subscriber = event_service.read().await.subscribe();

    let event_service_clone = Arc::clone(&event_service);

    tokio::spawn(async move {
        let event_service_clone2 = Arc::clone(&event_service_clone);
        while let Ok(event) = subscriber.recv().await {
            event_service_clone2.read().await.publish(&event).await;
        }
    });

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(Arc::clone(&game_manager), Arc::clone(&event_service), state.clone(), stream, addr));
    }

    Ok(())
}

async fn handle_connection(game_manager: Arc<RwLock<GameManager>>, event_service: Arc<RwLock<EventService>>, peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    // peer_map.lock().await.insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg|{
        let game_manager_clone = Arc::clone(&game_manager);
        let event_service_clone = Arc::clone(&event_service);
        let addr_clone = addr.clone();
        let tx_clone = tx.clone();

        async move {
        let addr = addr_clone.clone();
            let request = text_to_request(msg.to_text().unwrap());
            if let Err(error) = request {
                println!("{}", error);
                return Ok(());
            }
            match request.unwrap() {
                RequestEnum::AuthorizeWebsocketConnectionRequest(AuthorizeWebsocketConnectionRequest { game_id, user_id }) => {
                    authorize(Arc::clone(&game_manager_clone), Arc::clone(&event_service_clone), game_id, user_id, addr, tx_clone).await;
                },

                RequestEnum::MakeMoveRequest(MakeMoveRequest { game_id, user_id, from, to }) => {
                    let r = make_move(Arc::clone(&game_manager_clone), Arc::clone(&event_service_clone), game_id, user_id, from, to).await;
                    // return Ok(());
                },

                _ => {
                    println!("Unknown request");
                    return Ok(());
                },
            }
            Ok(())
        }
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().await.remove(&addr);
}

fn text_to_request(text: &str) -> Result<RequestEnum, String> {
    if let Ok(request) = from_str::<MakeMoveRequest>(text) {
        return Ok(RequestEnum::MakeMoveRequest(request));
    }
    if let Ok(request) = from_str::<AuthorizeWebsocketConnectionRequest>(text) {
        return Ok(RequestEnum::AuthorizeWebsocketConnectionRequest(request));
    }
    Err("Unknown request".to_string())
}

async fn authorize(
    game_manager: Arc<RwLock<GameManager>>,
    event_service: Arc<RwLock<EventService>>,
    game_id: Uuid,
    user_id: String,
    address: SocketAddr,
    unbounded_sender: Tx,
) {
    let result = game_manager.write().await.connection_manager.add_connection(&game_id, &user_id, Some(address), Some(Arc::new(Mutex::new(unbounded_sender))));
    match result {
        Ok(message) | Err(message) => {
            let response = Response::AuthorizeWebsocketConnectionResponse { game_id, user_id, connection_id: address, message };
            event_service.read().await.publish(&response).await;
        },
    }
}

async fn make_move(
    game_manager: Arc<RwLock<GameManager>>,
    event_service: Arc<RwLock<EventService>>,
    game_id: Uuid,
    user_id: String,
    from: String,
    to: String
) -> Response {
    let response = match game_manager.write().await.get_game_by_id(&game_id).await {
        Err(_) => Response::MakeMoveResponse {
            game_id,
            message: "Game does not exist".to_string(),
            columns: "".to_string(),
            rows: "".to_string(),
            board: HashMap::new(),
        },
        Ok(mut game) => {
            match game.get_board() {
                None => Response::MakeMoveResponse {
                    game_id,
                    message: "Game does not exist".to_string(),
                    columns: "".to_string(),
                    rows: "".to_string(),
                    board: HashMap::new(),
                },
                Some(board) => {
                    // self.game.as_mut().unwrap().get_board().unwrap().make_move_str("e2".to_string(), "e4".to_string());
                    board.make_move(from.clone(), to.clone());
                    let result = "Made move: ".to_string()
                        + from.clone().as_str()
                        + " "
                        + to.clone().as_str()
                        + "\n"
                        + board.board_to_string().as_str();
                    println!("{}", result);
                    Response::MakeMoveResponse {
                        game_id,
                        message: format!("Made move from {} to {}", from, to),
                        columns: board.get_columns(),
                        rows: board.get_rows(),
                        board: board_to_dict(board),
                    }
                }
            }
        },
    };

    event_service.read().await.publish(&response).await;
    response
}

fn board_to_dict(board: &mut Board) -> HashMap<String, (String, Vec<String>)> {
    let mut dict: HashMap<String, (String, Vec<String>)> = HashMap::new();
    // todo: add calculation of possible moves for each piece in given position

    for (coordinates, piece) in board.get_pieces_dict() {
        match piece {
            Some(p) => {
                let piece_possible_coordinates = p.get_possible_moves();
                let s = p.get_symbol();
                dict.insert(coordinates.deref().to_string(), (p.get_symbol(), piece_possible_coordinates.clone()));
            },
            _ => (),
        }
    }
    dict
}
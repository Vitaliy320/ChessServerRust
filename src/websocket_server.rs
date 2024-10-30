// use std::sync::Arc;
// use std::collections::HashMap;
// use tokio::net::TcpStream;
// use std::ops::Deref;
// use crate::chess_engine::board::Board;
// use futures_util::{SinkExt, StreamExt};
// use tokio::net::TcpListener;
// use tokio::sync::Mutex;
// use tokio_websockets::{ServerBuilder, WebSocketStream};
// use tokio::sync::RwLock;
// use uuid::Uuid;
// use crate::game_repository::GameRepository;
// use crate::game_manager::GameManager;
// use crate::request::{
//     RequestEnum,
//     GetGamesRequest,
//     CreateGameRequest,
//     JoinGameRequest,
//     AuthorizeWebsocketConnectionRequest,
//     MakeMoveRequest,
// };
// use crate::response::Response;
// use crate::connection_manager::ConnectionManager;
// use crate::event_service::{Event, EventService};
//
// pub async fn run_websocket_server(
//     game_manager: Arc<RwLock<GameManager>>,
// ) {
//     let event_service = Arc::new(Mutex::new(EventService::new(100, Arc::clone(&game_manager))));
//     let mut subscriber = event_service.lock().await.subscribe();
//
//     let event_service_clone = Arc::clone(&event_service);
//
//     tokio::spawn(async move {
//         let event_service_clone2 = Arc::clone(&event_service_clone);
//         while let Ok(event) = subscriber.recv().await {
//             event_service_clone2.lock().await.publish(&event).await;
//         }
//     });
//
//     let ws_listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
//     println!("Websocket server started");
//     loop {
//         let (conn, _) = ws_listener.accept().await.unwrap();
//         println!("Client connected");
//
//         let game_manager_clone = Arc::clone(&game_manager);
//         let event_service_clone_in_loop = Arc::clone(&event_service);
//
//         tokio::spawn(async move {
//             let mut ws = Arc::new(Mutex::new(ServerBuilder::new().accept(conn).await.unwrap()));
//
//             while let Some(Ok(item)) = ws.lock().await.next().await {
//                 println!("Received: {item:?}");
//                 let response = match item.as_text() {
//                     None => "Could not handle request".to_string(),
//                     Some(text) => {
//                         let ws_clone = Arc::clone(&ws);
//                         let response = handle_request(
//                             ws_clone,
//                             text,
//                             Arc::clone(&game_manager_clone),
//                         ).await;
//                         event_service_clone_in_loop.lock().await.publish(&response).await;
//                         serde_json::to_string(&response).unwrap()
//                     }
//                 };
//                 let m = tokio_websockets::Message::text(response);
//                 ws.lock().await.send(m).await.unwrap();
//             }
//         });
//     }
// }
//
//
// pub async fn handle_request(
//     ws: Arc<Mutex<WebSocketStream<TcpStream>>>,
//     request: &str,
//     game_manager: Arc<RwLock<GameManager>>,
// ) -> Response {
//     let mut game_manager_lock = game_manager.write().await;
//     if let Ok(req) = serde_json::from_str::<AuthorizeWebsocketConnectionRequest>(request) {
//         let connection_id = Uuid::new_v4();
//         let res = game_manager_lock.connection_manager.add_connection(&req.game_id, &req.user_id, Some(connection_id), Some(ws));
//         // let resp = Response::AuthorizeWebsocketConnectionResponse { message: res.unwrap()};
//         return match res {
//             Ok(message) | Err(message) => Response::AuthorizeWebsocketConnectionResponse {
//                 game_id: req.game_id,
//                 user_id: req.user_id,
//                 connection_id,
//                 message,
//             },
//         }
//     };
//
//     if let Ok(req) = serde_json::from_str::<MakeMoveRequest>(request) {
//         return make_move(Arc::clone(&game_manager), req.game_id, req.user_id, req.from, req.to).await;
//     };
//
//     Response::RequestFailedResponse { message: "Unknown request".to_string() }
//
//     // match serde_json::from_str::<RequestEnum>(request) {
//     //     Ok(RequestEnum::AuthorizeWebsocketConnectionRequest (AuthorizeWebsocketConnectionRequest { game_id, user_id })) => {
//     //         let connection_id = Some(Uuid::new_v4());
//     //         let res = game_manager_lock.connection_manager.add_connection(&game_id, &user_id, connection_id, Some(ws));
//     //         Response::AuthorizeWebsocketConnectionResponse { message: res.unwrap() }
//     //     },
//     //     Ok(RequestEnum::MakeMoveRequest (MakeMoveRequest { game_id, user_id, from, to })) => {
//     //         make_move(Arc::clone(&game_manager), game_id, user_id, from, to).await
//     //     },
//     //     _ => Response::RequestFailedResponse { message: "Unknown websocket request".to_string() },
//     // }
// }
//
// // async fn authorize()
//
// async fn make_move(game_manager: Arc<RwLock<GameManager>>, game_id: Uuid, user_id: String, from: String, to: String) -> Response {
//     let mut game_manager_lock = game_manager.write().await;
//
//     match game_manager_lock.get_game_by_id(&game_id).await {
//         Err(_) => Response::MakeMoveResponse {
//             game_id,
//             message: "Game does not exist".to_string(),
//             columns: "".to_string(),
//             rows: "".to_string(),
//             board: HashMap::new(),
//         },
//         Ok(mut game) => {
//             match game.get_board() {
//                 None => Response::MakeMoveResponse {
//                     game_id,
//                     message: "Game does not exist".to_string(),
//                     columns: "".to_string(),
//                     rows: "".to_string(),
//                     board: HashMap::new(),
//                 },
//                 Some(board) => {
//                     // self.game.as_mut().unwrap().get_board().unwrap().make_move_str("e2".to_string(), "e4".to_string());
//                     board.make_move(from.clone(), to.clone());
//                     let result = "Made move: ".to_string()
//                         + from.clone().as_str()
//                         + " "
//                         + to.clone().as_str()
//                         + "\n"
//                         + board.board_to_string().as_str();
//                     println!("{}", result);
//                     Response::MakeMoveResponse {
//                         game_id,
//                         message: format!("Made move from {} to {}", from, to),
//                         columns: board.get_columns(),
//                         rows: board.get_rows(),
//                         board: board_to_dict(board),
//                     }
//                 }
//             }
//         },
//     }
// }
//
// fn board_to_dict(board: &mut Board) -> HashMap<String, (String, Vec<String>)> {
//     let mut dict: HashMap<String, (String, Vec<String>)> = HashMap::new();
//     // todo: add calculation of possible moves for each piece in given position
//
//     for (coordinates, piece) in board.get_pieces_dict() {
//         match piece {
//             Some(p) => {
//                 let piece_possible_coordinates = p.get_possible_moves();
//                 let s = p.get_symbol();
//                 dict.insert(coordinates.deref().to_string(), (p.get_symbol(), piece_possible_coordinates.clone()));
//             },
//             _ => (),
//         }
//     }
//     dict
// }

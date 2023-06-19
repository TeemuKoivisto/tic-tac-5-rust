use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use quick_protobuf::{BytesReader, MessageRead};
use tic_tac_5::proto::proto_all::*;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::state::client::Client;
use crate::state::events::{ClientEvent, GameEvent, LobbyEvent};
use crate::ws::serialize_server_event::serialize_server_event;

pub struct SubscribedGame {
    game_id: String,
    sender: broadcast::Sender<ClientEvent>,
}

pub struct WsSession {
    client: Client,
    pub socket_id: u32,
    ws_sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    ws_receiver: SplitStream<WebSocketStream<TcpStream>>,
    client_sender: broadcast::Sender<ClientEvent>,
    lobby_receiver: broadcast::Receiver<LobbyEvent>,
    game_sender: broadcast::Sender<GameEvent>,
    game_receiver: broadcast::Receiver<GameEvent>,
    subscribed_lobby: Option<broadcast::Sender<ClientEvent>>,
    subscribed_games: Vec<SubscribedGame>,
}

impl WsSession {
    pub fn new(
        socket_id: u32,
        socket: WebSocketStream<TcpStream>,
        client_sender: broadcast::Sender<ClientEvent>,
        lobby_receiver: broadcast::Receiver<LobbyEvent>,
        game_sender: broadcast::Sender<GameEvent>,
        game_receiver: broadcast::Receiver<GameEvent>,
    ) -> Self {
        let (ws_sender, ws_receiver) = socket.split();
        Self {
            client: Client {
                name: "".to_string(),
                player_id: 0,
                socket_id,
            },
            socket_id,
            ws_sender,
            ws_receiver,
            client_sender,
            lobby_receiver,
            game_sender,
            game_receiver,
            subscribed_lobby: None,
            subscribed_games: Vec::new(),
        }
    }

    pub fn send_disconnect(&mut self) {
        self.send_to_lobby(ClientEvent::Disconnected(self.socket_id));
        self.send_to_game(ClientEvent::Disconnected(self.socket_id));
    }

    fn set_player(&mut self, player_join: &PlayerJoinLobby) {
        self.client = Client {
            name: player_join.name.clone(),
            player_id: player_join.player_id,
            socket_id: self.socket_id,
        }
    }

    pub async fn handle_ws_message(
        &mut self,
        msg: Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg = msg.into_data();
        let message_type = msg.remove(0);
        let mut reader = BytesReader::from_bytes(&msg);
        // debug!("New message {}", message_type);
        match ClientMsgType::try_from(message_type) {
            Ok(ClientMsgType::join_lobby) => {
                if let Ok(player_join) = PlayerJoinLobby::from_reader(&mut reader, &msg) {
                    debug!("ClientMsgType::join_lobby {:#?}", player_join);
                    // ctx.join_lobby(socket_id, player_join).await;
                    // ctx.broadcast_lobby_state().await;
                    // println!("sending to {} subscribers", self.subscribed_games.len());
                    self.set_player(&player_join);
                    self.send_to_lobby(ClientEvent::PlayerJoinLobby(player_join));
                }
            }
            Ok(ClientMsgType::create_lobby_game) => {
                if let Ok(payload) = PlayerCreateGame::from_reader(&mut reader, &msg) {
                    debug!("ClientMsgType::create_lobby_game {:#?}", payload);
                    self.send_to_lobby(ClientEvent::PlayerCreateGame(self.socket_id, payload));
                    // let (started, game_mut) = ctx.create_lobby_game(socket_id, create_game).await;
                    // game_mut
                    //     .lock()
                    //     .await
                    //     .add_player_connection(socket_id, conn.clone());
                    // player.set_joined_game(game_mut.clone()).await;
                    // if started {
                    //     ctx.start_game(game_mut).await;
                    // }
                    // ctx.broadcast_lobby_state().await;
                }
            }
            Ok(ClientMsgType::join_lobby_game) => {
                if let Ok(player_join) = PlayerJoinGame::from_reader(&mut reader, &msg) {
                    debug!("ClientMsgType::join_lobby_game {:#?}", player_join);
                    self.send_to_lobby(ClientEvent::PlayerJoinGame(self.socket_id, player_join));
                    // let (started, game_mut) = ctx.join_lobby_game(socket_id, player_join).await;
                    // game_mut
                    //     .lock()
                    //     .await
                    //     .add_player_connection(socket_id, conn.clone());
                    // player.set_joined_game(game_mut.clone()).await;
                    // if started {
                    //     ctx.start_game(game_mut).await;
                    // }
                    // ctx.broadcast_lobby_state().await;
                }
            }
            Ok(ClientMsgType::player_select_cell) => {
                if let Ok(payload) = PlayerSelectCell::from_reader(&mut reader, &msg) {
                    debug!("ClientMsgType::player_select_cell {:#?}", payload);
                    self.send_to_game(ClientEvent::SelectCell(self.socket_id, payload));
                    // let ended = player.player_select_cell(&payload).await;
                    // if ended {
                    //     player.remove_joined_game().await;
                    //     let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                    //     ctx.end_game(game_id).await;
                    //     ctx.remove_game(game_id).await;
                    //     ctx.broadcast_lobby_state().await;
                    // }
                }
            }
            Ok(ClientMsgType::leave_game) => {
                if let Ok(payload) = PlayerLeaveGame::from_reader(&mut reader, &msg) {
                    debug!("ClientMsgType::player_leave {:#?}", payload);
                    // let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                    // let ended = ctx.player_leave_game(socket_id, payload).await;
                    // if ended {
                    //     player.remove_joined_game().await;
                    //     ctx.end_game(game_id).await;
                    //     ctx.remove_game(game_id).await;
                    // }
                    // ctx.broadcast_lobby_state().await;
                }
            }
            _ => error!("Unknown header: {}", message_type),
        };
        Ok(())
    }
    pub async fn handle_lobby_event(&mut self, msg: LobbyEvent) {
        info!("Client {} -> LobbyEvent {:?}", self.socket_id, msg);
        match msg {
            LobbyEvent::Subscribe(sender) => {
                self.subscribed_lobby = Some(sender);
            }
            LobbyEvent::JoinLobby(_) => todo!(),
            LobbyEvent::LobbyMsg(_) => todo!(),
            LobbyEvent::LeaveLobby(payload) => {}
            LobbyEvent::LobbyState(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::lobby_state, &payload))
                    .await;
            }
            LobbyEvent::JoinLobbyGame(_) => todo!(),
            LobbyEvent::LeaveLobbyGame(_) => todo!(),
            LobbyEvent::PlayerJoinGame(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::player_join, &payload))
                    .await;
            }
        }
    }
    pub async fn handle_game_event(&mut self, msg: GameEvent) {
        // info!("Client -> GameEvent {:?}", msg);
        info!("Client {} -> GameEvent", self.socket_id);
        match msg {
            GameEvent::Subscribe(game_id, client_sender) => {
                info!(
                    "ClientEvent::SubscribeToGame (socket_id {} game_id {})",
                    self.socket_id, game_id
                );
                let _ = client_sender.send(ClientEvent::SubscribeToGame(
                    self.client.clone(),
                    self.game_sender.clone(),
                ));
                self.subscribed_games.push(SubscribedGame {
                    game_id,
                    sender: client_sender,
                });
            }
            GameEvent::PlayerJoin(_) => todo!(),
            GameEvent::PlayerLeave() => todo!(),
            GameEvent::GameStart(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::game_start, &payload))
                    .await;
            }
            GameEvent::GameEnd(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::game_end, &payload))
                    .await;
            }
            GameEvent::GameUpdate(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(
                        ServerMsgType::game_player_move,
                        &GameMove {
                            player: payload.player_number,
                            x: payload.x,
                            y: payload.y,
                        },
                    ))
                    .await;
            }
        }
    }
    fn send_to_lobby(&mut self, event: ClientEvent) {
        let _ = self.subscribed_lobby.as_ref().unwrap().send(event.clone());
    }
    fn send_to_game(&mut self, event: ClientEvent) {
        self.subscribed_games
            .retain(|sub| sub.sender.send(event.clone()).is_ok());
    }
}

pub fn run_session(mut actor: WsSession) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = actor.ws_receiver.next() => {
                    match msg {
                        Ok(ev) => {
                            let mut stop = true;
                            if ev.is_binary() {
                                stop = actor.handle_ws_message(ev).await.is_err();
                            }
                            if stop {
                                break;
                            }
                        }
                        Err(err) => {
                            println!("socket error {:?}", err);
                            break;
                        }
                    }
                },
                Ok(ev) = actor.lobby_receiver.recv() => {
                    actor.handle_lobby_event(ev).await;
                },
                Ok(ev) = actor.game_receiver.recv() => {
                    actor.handle_game_event(ev).await;
                },
                else => {
                    break;
                },
            }
        }
        actor.send_disconnect();
    })
}

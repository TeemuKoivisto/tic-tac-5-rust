use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use quick_protobuf::{BytesReader, MessageRead};
use tic_tac_5::proto::{client_events::*, game::*, server_events::*};
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

use crate::state::client::Client;
use crate::state::events::{
    ClientToGameEvent, ClientToLobbyEvent, GameToClientEvent, LobbyToClientEvent,
};
use crate::ws::serialize_server_event::serialize_server_event;

use super::session_handle::SessionHandle;

pub struct SubscribedGame {
    game_id: String,
    sender: broadcast::Sender<ClientToGameEvent>,
}

pub struct Session {
    pub client: Client,
    pub socket_id: u32,
    ws_sender: SplitSink<WebSocket, Message>,
    ws_receiver: SplitStream<WebSocket>,
    client_sender: broadcast::Sender<ClientToLobbyEvent>,
    pub lobby_receiver: broadcast::Receiver<LobbyToClientEvent>,
    game_sender: broadcast::Sender<GameToClientEvent>,
    pub game_receiver: broadcast::Receiver<GameToClientEvent>,
    subscribed_lobby: Option<broadcast::Sender<ClientToLobbyEvent>>,
    subscribed_games: Vec<SubscribedGame>,
}

impl Session {
    pub fn new(
        socket_id: u32,
        socket: WebSocket,
        client_sender: broadcast::Sender<ClientToLobbyEvent>,
        lobby_receiver: broadcast::Receiver<LobbyToClientEvent>,
        game_sender: broadcast::Sender<GameToClientEvent>,
        game_receiver: broadcast::Receiver<GameToClientEvent>,
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

    pub fn restore(&mut self, socket: WebSocket) {
        let (ws_sender, ws_receiver) = socket.split();
        self.ws_sender = ws_sender;
        self.ws_receiver = ws_receiver;
        let _ = self.ws_sender.send(serialize_server_event(
            ServerMsgType::player_status,
            &PlayerStatus {
                waiting_games: self.get_game_ids(),
                ended_games: Vec::new(),
            },
        ));
    }

    pub fn send_disconnect(&mut self) {
        self.send_to_lobby(ClientToLobbyEvent::Disconnected(self.socket_id));
        self.send_to_game(ClientToGameEvent::Disconnected(
            self.socket_id,
            self.client.player_id,
        ));
    }

    fn set_player(&mut self, player_join: &PlayerJoinLobby) {
        self.client = Client {
            name: player_join.name.clone(),
            player_id: player_join.player_id,
            socket_id: self.socket_id,
        }
    }

    pub fn get_game_ids(&self) -> Vec<String> {
        self.subscribed_games
            .iter()
            .map(|g| g.game_id.clone())
            .collect()
    }

    pub async fn handle_ws_message(
        &mut self,
        msg: Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match msg {
            Message::Binary(mut raw_buf) => {
                // let mut msg = msg.into_data();
                let message_type = raw_buf.remove(0);
                let mut reader = BytesReader::from_bytes(&raw_buf);
                // debug!("New message {}", message_type);
                match ClientMsgType::try_from(message_type) {
                    Ok(ClientMsgType::join_lobby) => {
                        if let Ok(player_join) = PlayerJoinLobby::from_reader(&mut reader, &raw_buf)
                        {
                            debug!("ClientMsgType::join_lobby {:#?}", player_join);
                            self.set_player(&player_join);
                            self.send_to_lobby(ClientToLobbyEvent::PlayerJoinLobby(player_join));
                        }
                    }
                    Ok(ClientMsgType::create_lobby_game) => {
                        if let Ok(payload) = PlayerCreateGame::from_reader(&mut reader, &raw_buf) {
                            debug!("ClientMsgType::create_lobby_game {:#?}", payload);
                            self.send_to_lobby(ClientToLobbyEvent::PlayerCreateGame(
                                self.socket_id,
                                payload,
                            ));
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
                        if let Ok(player_join) = PlayerJoinGame::from_reader(&mut reader, &raw_buf)
                        {
                            debug!("ClientMsgType::join_lobby_game {:#?}", player_join);
                            self.send_to_lobby(ClientToLobbyEvent::PlayerJoinGame(
                                self.socket_id,
                                player_join,
                            ));
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
                        if let Ok(payload) = PlayerSelectCell::from_reader(&mut reader, &raw_buf) {
                            debug!("ClientMsgType::player_select_cell {:#?}", payload);
                            self.send_to_game(ClientToGameEvent::SelectCell(
                                self.socket_id,
                                payload,
                            ));
                        }
                    }
                    Ok(ClientMsgType::leave_game) => {
                        if let Ok(payload) = PlayerLeaveGame::from_reader(&mut reader, &raw_buf) {
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
                }
                Ok(())
            }
            Message::Text(_) => todo!(),
            Message::Ping(_) => todo!(),
            Message::Pong(_) => todo!(),
            Message::Close(_) => {
                // self.send(ClientToLobbyEvent::Disconnected(self.user_id));
                Err("Disconnected".into())
            }
        }
    }

    pub async fn handle_lobby_event(&mut self, msg: LobbyToClientEvent) {
        info!("Client {} -> LobbyToClientEvent {:?}", self.socket_id, msg);
        match msg {
            LobbyToClientEvent::Subscribe(sender) => {
                self.subscribed_lobby = Some(sender);
            }
            LobbyToClientEvent::JoinLobby(_) => todo!(),
            LobbyToClientEvent::LobbyMsg(_) => todo!(),
            LobbyToClientEvent::LeaveLobby(payload) => {
                // TODO send to client
            }
            LobbyToClientEvent::LobbyState(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::lobby_state, &payload))
                    .await;
            }
            LobbyToClientEvent::JoinLobbyGame(_) => todo!(),
            LobbyToClientEvent::LeaveLobbyGame(_) => todo!(),
            LobbyToClientEvent::PlayerJoinGame(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::player_join, &payload))
                    .await;
            }
        }
    }
    pub async fn handle_game_event(&mut self, msg: GameToClientEvent) {
        info!("Client {} -> GameToClientEvent", self.socket_id);
        match msg {
            GameToClientEvent::Subscribe(game_id, client_sender) => {
                info!(
                    "ClientEvent::SubscribeToGame (socket_id {} game_id {})",
                    self.socket_id, game_id
                );
                let _ = client_sender.send(ClientToGameEvent::SubscribeToGame(
                    self.client.clone(),
                    self.game_sender.clone(),
                ));
                self.subscribed_games.push(SubscribedGame {
                    game_id,
                    sender: client_sender,
                });
            }
            GameToClientEvent::PlayerDisconnected(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(
                        ServerMsgType::player_disconnected,
                        &payload,
                    ))
                    .await;
            }
            GameToClientEvent::PlayerReconnected(player_id) => {}
            GameToClientEvent::PlayerJoin(_) => todo!(),
            GameToClientEvent::PlayerLeave() => todo!(),
            GameToClientEvent::GameStart(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::game_start, &payload))
                    .await;
            }
            GameToClientEvent::GameEnd(payload) => {
                let _ = self
                    .ws_sender
                    .send(serialize_server_event(ServerMsgType::game_end, &payload))
                    .await;
            }
            GameToClientEvent::GameUpdate(payload) => {
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
    fn send_to_lobby(&mut self, event: ClientToLobbyEvent) {
        let _ = self.subscribed_lobby.as_ref().unwrap().send(event.clone());
    }
    fn send_to_game(&mut self, event: ClientToGameEvent) {
        self.subscribed_games
            .retain(|sub| sub.sender.send(event.clone()).is_ok());
    }
}

pub fn run_session(mut session: SessionHandle) -> JoinHandle<SessionHandle> {
    tokio::spawn(async move {
        let actor = &mut session.actor;
        loop {
            tokio::select! {
                Some(msg) = actor.ws_receiver.next() => {
                    match msg {
                        Ok(ev) => {
                            if actor.handle_ws_message(ev).await.is_err() {
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
        return session;
    })
}

use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};
use tic_tac_5::proto::{client_events::*, game::*, server_events::*};
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

use crate::state::events::{
    ClientToGameEvent, ClientToLobbyEvent, GameToClientEvent, LobbyToClientEvent, PlayerMove,
};
use crate::ws::serialize_server_event::serialize_server_event;

use super::session_handle::SessionHandle;
use super::session_state::SessionState;

pub struct Session {
    pub state: SessionState,
    pub socket_id: u32,
    ws_sender: SplitSink<WebSocket, Message>,
    ws_receiver: SplitStream<WebSocket>,
    client_sender: broadcast::Sender<ClientToLobbyEvent>,
    pub lobby_receiver: broadcast::Receiver<LobbyToClientEvent>,
    game_sender: broadcast::Sender<GameToClientEvent>,
    pub game_receiver: broadcast::Receiver<GameToClientEvent>,
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
            state: SessionState::new(socket_id),
            socket_id,
            ws_sender,
            ws_receiver,
            client_sender,
            lobby_receiver,
            game_sender,
            game_receiver,
        }
    }

    pub async fn restore(&mut self, socket: WebSocket) {
        let (ws_sender, ws_receiver) = socket.split();
        self.ws_sender = ws_sender;
        self.ws_receiver = ws_receiver;
        self.send_to_ws(ServerMsgType::player_status, &self.state.get_player_state())
            .await;
    }

    pub fn send_disconnect(&mut self) {
        self.state.transit(PlayerAppState::disconnected);
        self.send_to_lobby(ClientToLobbyEvent::Disconnected(self.socket_id));
        self.send_to_game(ClientToGameEvent::Disconnected(
            self.socket_id,
            self.state.player_id,
        ));
    }

    pub async fn handle_ws_message(
        &mut self,
        msg: Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match msg {
            Message::Binary(mut raw_buf) => {
                let message_type = raw_buf.remove(0);
                let mut reader = BytesReader::from_bytes(&raw_buf);
                match ClientMsgType::try_from(message_type) {
                    Ok(ClientMsgType::join_lobby) => {
                        if let Ok(player_join) = PlayerJoinLobby::from_reader(&mut reader, &raw_buf)
                        {
                            debug!("ClientMsgType::join_lobby {:#?}", player_join);
                            self.state.set_player(&player_join);
                            self.send_to_lobby(ClientToLobbyEvent::PlayerJoinLobby(player_join));
                            self.state.transit(PlayerAppState::lobby);
                        }
                    }
                    Ok(ClientMsgType::create_lobby_game) => {
                        if let Ok(payload) = PlayerCreateGame::from_reader(&mut reader, &raw_buf) {
                            debug!("ClientMsgType::create_lobby_game {:#?}", payload);
                            self.state.transit(PlayerAppState::waiting_game_start);
                            self.send_to_lobby(ClientToLobbyEvent::PlayerCreateGame(
                                self.socket_id,
                                payload,
                            ));
                        }
                    }
                    Ok(ClientMsgType::join_lobby_game) => {
                        if let Ok(player_join) = PlayerJoinGame::from_reader(&mut reader, &raw_buf)
                        {
                            debug!("ClientMsgType::join_lobby_game {:#?}", player_join);
                            self.state.transit(PlayerAppState::waiting_game_start);
                            self.send_to_lobby(ClientToLobbyEvent::PlayerJoinGame(
                                self.socket_id,
                                player_join,
                            ));
                        }
                    }
                    Ok(ClientMsgType::player_select_cell) => {
                        if let Ok(payload) = PlayerSelectCell::from_reader(&mut reader, &raw_buf) {
                            debug!("ClientMsgType::player_select_cell {:#?}", payload);
                            self.send_to_game(ClientToGameEvent::SelectCell(
                                self.socket_id,
                                PlayerMove {
                                    player_id: self.state.player_id,
                                    x: payload.x,
                                    y: payload.y,
                                },
                            ));
                        }
                    }
                    Ok(ClientMsgType::player_rejoin) => {
                        if let Ok(payload) = PlayerRejoinGame::from_reader(&mut reader, &raw_buf) {
                            debug!("ClientMsgType::player_rejoin {:#?}", payload);
                            self.send_to_game(ClientToGameEvent::Reconnected(
                                self.socket_id,
                                self.state.player_id,
                            ));
                            self.send_to_ws(ServerMsgType::player_reconnected, &payload)
                                .await;
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
            Message::Close(_) => Err("Disconnected".into()),
        }
    }

    pub async fn handle_lobby_event(&mut self, msg: LobbyToClientEvent) {
        info!("Client {} -> LobbyToClientEvent {:?}", self.socket_id, msg);
        match msg {
            LobbyToClientEvent::Subscribe(sender) => {
                self.state.set_lobby(sender);
                self.state.transit(PlayerAppState::lobby);
            }
            LobbyToClientEvent::JoinLobby(_) => todo!(),
            LobbyToClientEvent::LobbyMsg(_) => todo!(),
            LobbyToClientEvent::LeaveLobby(payload) => {
                // let was_player = payload.iter().find(|s| s == &&self.socket_id);
                // if
            }
            LobbyToClientEvent::LobbyState(payload) => {
                self.send_to_ws(ServerMsgType::lobby_state, &payload).await;
            }
            LobbyToClientEvent::JoinLobbyGame(_) => todo!(),
            LobbyToClientEvent::LeaveLobbyGame(_) => todo!(),
            LobbyToClientEvent::PlayerJoinGame(payload) => {
                self.send_to_ws(ServerMsgType::player_join, &payload).await;
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
                self.state.transit(PlayerAppState::waiting_game_start);
                let _ = client_sender.send(ClientToGameEvent::SubscribeToGame(
                    self.state.get_client(),
                    self.game_sender.clone(),
                ));
                self.state.push_game(game_id, client_sender);
            }
            GameToClientEvent::PlayerDisconnected(payload) => {
                self.state.transit_game(PlayerInGameState::waiting_player);
                self.send_to_ws(ServerMsgType::player_disconnected, &payload)
                    .await;
            }
            GameToClientEvent::PlayerReconnected(payload) => {
                self.send_to_ws(ServerMsgType::player_reconnected, &payload)
                    .await;
            }
            GameToClientEvent::PlayerJoin(_) => todo!(),
            GameToClientEvent::PlayerLeave() => todo!(),
            GameToClientEvent::GameStart(payload) => {
                // @TODO use the payload
                self.state.transit(PlayerAppState::in_game);
                self.state.transit_game(PlayerInGameState::x_turn);
                self.send_to_ws(ServerMsgType::game_start, &payload).await;
            }
            GameToClientEvent::GameEnd(payload) => {
                self.state.transit_game(PlayerInGameState::ended);
                self.send_to_ws(ServerMsgType::game_end, &payload).await;
            }
            GameToClientEvent::GameUpdate(payload) => {
                // self.state.transit_game(to);
                // if self.game_state == PlayerInGameState::x_turn {
                //     self.game_state = PlayerInGameState::o_turn;
                // } else {
                //     self.game_state = PlayerInGameState::x_turn;
                // }
                self.send_to_ws(ServerMsgType::game_player_move, &payload)
                    .await;
            }
        }
    }
    async fn send_to_ws<M: MessageWrite>(&mut self, header: ServerMsgType, payload: &M) {
        let _ = self
            .ws_sender
            .send(serialize_server_event(header, payload))
            .await;
    }
    fn send_to_lobby(&mut self, event: ClientToLobbyEvent) {
        let _ = self
            .state
            .subscribed_lobby
            .as_ref()
            .unwrap()
            .send(event.clone());
    }
    fn send_to_game(&mut self, event: ClientToGameEvent) {
        self.state
            .subscribed_games
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
                        Err(_err) => {
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

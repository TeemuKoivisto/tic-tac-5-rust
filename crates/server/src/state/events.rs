use tic_tac_5::proto::{client_events::*, game::*, server_events::*};
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use super::client::Client;

#[derive(Debug, Clone)]
pub enum LobbyToClientEvent {
    Subscribe(broadcast::Sender<ClientToLobbyEvent>),
    JoinLobby(u32),
    LobbyMsg(u32),
    LeaveLobby(Vec<u32>),
    LobbyState(LobbyState),
    JoinLobbyGame(u32),
    LeaveLobbyGame(u32),
    PlayerJoinGame(PlayerJoinGame),
}

#[derive(Debug, Clone)]
pub enum ClientToLobbyEvent {
    Connected(
        u32,
        Vec<String>,
        broadcast::Sender<LobbyToClientEvent>,
        broadcast::Sender<GameToClientEvent>,
    ),
    Disconnected(u32),
    PlayerJoinLobby(PlayerJoinLobby),
    PlayerCreateGame(u32, PlayerCreateGame),
    PlayerJoinGame(u32, PlayerJoinGame),
}

#[derive(Debug, Clone)]
pub struct PlayerMove {
    pub player_id: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub enum ClientToGameEvent {
    Reconnected(u32, u32),
    Disconnected(u32, u32),
    SubscribeToGame(Client, broadcast::Sender<GameToClientEvent>),
    SelectCell(u32, PlayerMove),
    LeaveGame(),
}

#[derive(Debug, Clone)]
pub enum GameToClientEvent {
    Subscribe(String, broadcast::Sender<ClientToGameEvent>),
    PlayerDisconnected(GamePlayerDisconnected),
    PlayerReconnected(GamePlayerReconnected),
    PlayerJoin(PlayerJoinGame),
    PlayerLeave(),
    GameStart(BoardState),
    GameEnd(GameEnd),
    GameUpdate(GameMove),
}

#[derive(Debug, Clone)]
pub enum GameToLobbyEvent {
    GameEnded(Uuid),
}

use tic_tac_5::proto::proto_all::*;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use super::client::Client;

// join_lobby = 0;
// lobby_msg = 1;
// create_lobby_game = 2;
// join_lobby_game = 3;
// leave_lobby_game = 4;
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

// player_select_cell = 5;
// leave_game = 6;
#[derive(Debug, Clone)]
pub enum ClientToLobbyEvent {
    Connected(
        u32,
        broadcast::Sender<LobbyToClientEvent>,
        broadcast::Sender<GameToClientEvent>,
    ),
    Disconnected(u32),
    PlayerJoinLobby(PlayerJoinLobby),
    PlayerCreateGame(u32, PlayerCreateGame),
    PlayerJoinGame(u32, PlayerJoinGame),
}

#[derive(Debug, Clone)]
pub enum ClientToGameEvent {
    Connected(u32),
    Disconnected(u32),
    SubscribeToGame(Client, broadcast::Sender<GameToClientEvent>),
    SelectCell(u32, PlayerSelectCell),
    LeaveGame(),
}

// player_join = 5;
// player_left = 6;
// game_start = 7;
// game_end = 8;
// game_player_move = 9;
#[derive(Debug, Clone)]
pub enum GameToClientEvent {
    Subscribe(String, broadcast::Sender<ClientToGameEvent>),
    PlayerJoin(PlayerJoinGame),
    PlayerLeave(),
    GameStart(GameStart),
    GameEnd(GameEnd),
    GameUpdate(PlayerSelectCell),
}

#[derive(Debug, Clone)]
pub enum GameToLobbyEvent {
    GameEnded(Uuid),
}

use tic_tac_5::proto::proto_all::*;
use tokio::sync::{broadcast, mpsc};

// join_lobby = 0;
// lobby_msg = 1;
// create_lobby_game = 2;
// join_lobby_game = 3;
// leave_lobby_game = 4;
#[derive(Debug, Clone)]
pub enum LobbyEvent {
    Subscribe(broadcast::Sender<ClientEvent>),
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
pub enum ClientEvent {
    Connected(
        u32,
        broadcast::Sender<LobbyEvent>,
        broadcast::Sender<GameEvent>,
    ),
    Disconnected(u32),
    PlayerJoinLobby(PlayerJoinLobby),
    PlayerCreateGame(u32, PlayerCreateGame),
    PlayerJoinGame(u32, PlayerJoinGame),
    SelectCell(),
    LeaveGame(),
}

// player_join = 5;
// player_left = 6;
// game_start = 7;
// game_end = 8;
// game_player_move = 9;
#[derive(Debug, Clone)]
pub enum GameEvent {
    Subscribe(String, broadcast::Sender<ClientEvent>),
    PlayerJoin(PlayerJoinGame),
    PlayerLeave(),
    GameStart(),
    GameEnd(),
    GameUpdate(),
}

use crate::proto::client_events::*;
use crate::proto::server_events::*;

// NOTE: must match packages/prototypes/protos/client_events.proto ClientMsgType
impl TryFrom<u8> for ClientMsgType {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == ClientMsgType::join_lobby as u8 => Ok(ClientMsgType::join_lobby),
            x if x == ClientMsgType::lobby_msg as u8 => Ok(ClientMsgType::lobby_msg),
            x if x == ClientMsgType::create_lobby_game as u8 => {
                Ok(ClientMsgType::create_lobby_game)
            }
            x if x == ClientMsgType::join_lobby_game as u8 => Ok(ClientMsgType::join_lobby_game),
            x if x == ClientMsgType::leave_lobby_game as u8 => Ok(ClientMsgType::leave_lobby_game),
            x if x == ClientMsgType::player_select_cell as u8 => {
                Ok(ClientMsgType::player_select_cell)
            }
            x if x == ClientMsgType::player_rejoin as u8 => Ok(ClientMsgType::player_rejoin),
            x if x == ClientMsgType::leave_game as u8 => Ok(ClientMsgType::leave_game),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum GameEvent {
    Move(PlayerSelectCell),
}

// NOTE: must match packages/prototypes/protos/server_events.proto ServerMsgType
#[derive(Debug)]
pub enum ServerEvent {
    ClientConnected(u32),
    ClientDisconnected(u32),
    // lobby
    LobbyGames(LobbyState),
    PlayerMsg(String),
    PlayerJoinLobby(u32),
    PlayerCreateGame(PlayerCreateGame),
    PlayerLeaveLobby(u32),
    LobbyGameUpdated(u32),
    // game
    PlayerJoin(PlayerJoinGame),
    GameStart(GameStart),
    GameEnd(GameEnd),
    GameMove(String, GameMove),
    Quit(PlayerLeaveGame),
}

impl TryInto<u8> for ServerMsgType {
    type Error = ();
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Self::lobby_state => Ok(0),
            Self::player_status => Ok(1),
            Self::player_msg => Ok(2),
            Self::player_join_lobby => Ok(3),
            Self::player_leave_lobby => Ok(4),
            Self::lobby_game_updated => Ok(5),
            Self::player_join => Ok(6),
            Self::player_left => Ok(7),
            Self::player_disconnected => Ok(8),
            Self::player_reconnected => Ok(9),
            Self::game_start => Ok(10),
            Self::game_end => Ok(11),
            Self::game_player_move => Ok(12),
        }
    }
}

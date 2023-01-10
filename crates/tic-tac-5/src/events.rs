use crate::proto::proto_all::*;

// NOTE: must match packages/prototypes/protos/proto_all.proto ClientMsgType
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
            x if x == ClientMsgType::player_move as u8 => Ok(ClientMsgType::player_move),
            x if x == ClientMsgType::leave_game as u8 => Ok(ClientMsgType::leave_game),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum GameEvent {
    Move(PlayerMove),
}

// NOTE: must match packages/prototypes/protos/proto_all.proto ServerMsgType
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
    Tick(Tick),
    Quit(PlayerLeave),
}

impl TryInto<u8> for ServerMsgType {
    type Error = ();
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Self::lobby_state => Ok(0),
            Self::player_msg => Ok(1),
            Self::player_join_lobby => Ok(2),
            Self::player_leave_lobby => Ok(3),
            Self::lobby_game_updated => Ok(4),
            Self::player_join => Ok(5),
            Self::game_start => Ok(6),
            Self::game_end => Ok(7),
            Self::tick => Ok(8),
            Self::player_left => Ok(9),
        }
    }
}

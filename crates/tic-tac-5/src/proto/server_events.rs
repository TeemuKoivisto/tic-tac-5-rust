// Automatically generated rust module for 'server_events.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerMsgType {
    player_state = 0,
    player_disconnected = 1,
    player_reconnected = 2,
    lobby_state = 3,
    player_msg = 4,
    player_join_lobby = 5,
    player_leave_lobby = 6,
    lobby_game_updated = 7,
    player_joined_game = 8,
    player_left_game = 9,
    game_start = 10,
    game_player_move = 11,
    game_end = 12,
}

impl Default for ServerMsgType {
    fn default() -> Self {
        ServerMsgType::player_state
    }
}

impl From<i32> for ServerMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => ServerMsgType::player_state,
            1 => ServerMsgType::player_disconnected,
            2 => ServerMsgType::player_reconnected,
            3 => ServerMsgType::lobby_state,
            4 => ServerMsgType::player_msg,
            5 => ServerMsgType::player_join_lobby,
            6 => ServerMsgType::player_leave_lobby,
            7 => ServerMsgType::lobby_game_updated,
            8 => ServerMsgType::player_joined_game,
            9 => ServerMsgType::player_left_game,
            10 => ServerMsgType::game_start,
            11 => ServerMsgType::game_player_move,
            12 => ServerMsgType::game_end,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ServerMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "player_state" => ServerMsgType::player_state,
            "player_disconnected" => ServerMsgType::player_disconnected,
            "player_reconnected" => ServerMsgType::player_reconnected,
            "lobby_state" => ServerMsgType::lobby_state,
            "player_msg" => ServerMsgType::player_msg,
            "player_join_lobby" => ServerMsgType::player_join_lobby,
            "player_leave_lobby" => ServerMsgType::player_leave_lobby,
            "lobby_game_updated" => ServerMsgType::lobby_game_updated,
            "player_joined_game" => ServerMsgType::player_joined_game,
            "player_left_game" => ServerMsgType::player_left_game,
            "game_start" => ServerMsgType::game_start,
            "game_player_move" => ServerMsgType::game_player_move,
            "game_end" => ServerMsgType::game_end,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerAppState {
    initializing = 0,
    disconnected = 1,
    lobby = 2,
    waiting_game_start = 3,
    in_game = 4,
    errored = 5,
}

impl Default for PlayerAppState {
    fn default() -> Self {
        PlayerAppState::initializing
    }
}

impl From<i32> for PlayerAppState {
    fn from(i: i32) -> Self {
        match i {
            0 => PlayerAppState::initializing,
            1 => PlayerAppState::disconnected,
            2 => PlayerAppState::lobby,
            3 => PlayerAppState::waiting_game_start,
            4 => PlayerAppState::in_game,
            5 => PlayerAppState::errored,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PlayerAppState {
    fn from(s: &'a str) -> Self {
        match s {
            "initializing" => PlayerAppState::initializing,
            "disconnected" => PlayerAppState::disconnected,
            "lobby" => PlayerAppState::lobby,
            "waiting_game_start" => PlayerAppState::waiting_game_start,
            "in_game" => PlayerAppState::in_game,
            "errored" => PlayerAppState::errored,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerInGameState {
    not_started = 0,
    x_turn = 1,
    o_turn = 2,
    waiting_player = 3,
    paused = 4,
    ended = 5,
}

impl Default for PlayerInGameState {
    fn default() -> Self {
        PlayerInGameState::not_started
    }
}

impl From<i32> for PlayerInGameState {
    fn from(i: i32) -> Self {
        match i {
            0 => PlayerInGameState::not_started,
            1 => PlayerInGameState::x_turn,
            2 => PlayerInGameState::o_turn,
            3 => PlayerInGameState::waiting_player,
            4 => PlayerInGameState::paused,
            5 => PlayerInGameState::ended,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PlayerInGameState {
    fn from(s: &'a str) -> Self {
        match s {
            "not_started" => PlayerInGameState::not_started,
            "x_turn" => PlayerInGameState::x_turn,
            "o_turn" => PlayerInGameState::o_turn,
            "waiting_player" => PlayerInGameState::waiting_player,
            "paused" => PlayerInGameState::paused,
            "ended" => PlayerInGameState::ended,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LobbyPlayer {
    pub player_id: u32,
    pub name: String,
}

impl<'a> MessageRead<'a> for LobbyPlayer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LobbyPlayer {
    fn get_size(&self) -> usize {
        0
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.name != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LobbyState {
    pub games: Vec<game::LobbyGame>,
    pub players: Vec<LobbyPlayer>,
}

impl<'a> MessageRead<'a> for LobbyState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.games.push(r.read_message::<game::LobbyGame>(bytes)?),
                Ok(18) => msg.players.push(r.read_message::<LobbyPlayer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LobbyState {
    fn get_size(&self) -> usize {
        0
        + self.games.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.players.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.games { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.players { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerState {
    pub app_state: PlayerAppState,
    pub game_state: PlayerInGameState,
    pub waiting_games: Vec<String>,
    pub ended_games: Vec<GameEnd>,
}

impl<'a> MessageRead<'a> for PlayerState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.app_state = r.read_enum(bytes)?,
                Ok(16) => msg.game_state = r.read_enum(bytes)?,
                Ok(26) => msg.waiting_games.push(r.read_string(bytes)?.to_owned()),
                Ok(34) => msg.ended_games.push(r.read_message::<GameEnd>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerState {
    fn get_size(&self) -> usize {
        0
        + if self.app_state == server_events::PlayerAppState::initializing { 0 } else { 1 + sizeof_varint(*(&self.app_state) as u64) }
        + if self.game_state == server_events::PlayerInGameState::not_started { 0 } else { 1 + sizeof_varint(*(&self.game_state) as u64) }
        + self.waiting_games.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.ended_games.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.app_state != server_events::PlayerAppState::initializing { w.write_with_tag(8, |w| w.write_enum(*&self.app_state as i32))?; }
        if self.game_state != server_events::PlayerInGameState::not_started { w.write_with_tag(16, |w| w.write_enum(*&self.game_state as i32))?; }
        for s in &self.waiting_games { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.ended_games { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerJoinedGame {
    pub game_id: String,
    pub state: PlayerAppState,
}

impl<'a> MessageRead<'a> for PlayerJoinedGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.state = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerJoinedGame {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.state == server_events::PlayerAppState::initializing { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.state != server_events::PlayerAppState::initializing { w.write_with_tag(16, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BoardState {
    pub game_id: String,
    pub player_in_turn: u32,
    pub start_time: u64,
    pub turns_elapsed: u32,
    pub players: Vec<game::Player>,
    pub cells: Vec<game::Cell>,
    pub state: PlayerInGameState,
}

impl<'a> MessageRead<'a> for BoardState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_in_turn = r.read_uint32(bytes)?,
                Ok(24) => msg.start_time = r.read_uint64(bytes)?,
                Ok(32) => msg.turns_elapsed = r.read_uint32(bytes)?,
                Ok(42) => msg.players.push(r.read_message::<game::Player>(bytes)?),
                Ok(50) => msg.cells.push(r.read_message::<game::Cell>(bytes)?),
                Ok(56) => msg.state = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BoardState {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.player_in_turn == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_in_turn) as u64) }
        + if self.start_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.start_time) as u64) }
        + if self.turns_elapsed == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.turns_elapsed) as u64) }
        + self.players.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.cells.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.state == server_events::PlayerInGameState::not_started { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_in_turn != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_in_turn))?; }
        if self.start_time != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.start_time))?; }
        if self.turns_elapsed != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.turns_elapsed))?; }
        for s in &self.players { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.cells { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.state != server_events::PlayerInGameState::not_started { w.write_with_tag(56, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameEnd {
    pub game_id: String,
    pub result: game::GameStatus,
    pub winner_id: u32,
    pub state: PlayerInGameState,
}

impl<'a> MessageRead<'a> for GameEnd {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.result = r.read_enum(bytes)?,
                Ok(24) => msg.winner_id = r.read_uint32(bytes)?,
                Ok(32) => msg.state = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameEnd {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.result == game::GameStatus::WAITING { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.winner_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.winner_id) as u64) }
        + if self.state == server_events::PlayerInGameState::not_started { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.result != game::GameStatus::WAITING { w.write_with_tag(16, |w| w.write_enum(*&self.result as i32))?; }
        if self.winner_id != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.winner_id))?; }
        if self.state != server_events::PlayerInGameState::not_started { w.write_with_tag(32, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameMove {
    pub player_number: u32,
    pub next_in_turn: u32,
    pub x: u32,
    pub y: u32,
    pub state: PlayerInGameState,
}

impl<'a> MessageRead<'a> for GameMove {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_number = r.read_uint32(bytes)?,
                Ok(16) => msg.next_in_turn = r.read_uint32(bytes)?,
                Ok(24) => msg.x = r.read_uint32(bytes)?,
                Ok(32) => msg.y = r.read_uint32(bytes)?,
                Ok(40) => msg.state = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameMove {
    fn get_size(&self) -> usize {
        0
        + if self.player_number == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_number) as u64) }
        + if self.next_in_turn == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.next_in_turn) as u64) }
        + if self.x == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
        + if self.state == server_events::PlayerInGameState::not_started { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player_number != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.player_number))?; }
        if self.next_in_turn != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.next_in_turn))?; }
        if self.x != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.y))?; }
        if self.state != server_events::PlayerInGameState::not_started { w.write_with_tag(40, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GamePlayerDisconnected {
    pub game_id: String,
    pub player_id: u32,
    pub symbol: String,
    pub name: String,
    pub state: PlayerInGameState,
}

impl<'a> MessageRead<'a> for GamePlayerDisconnected {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
                Ok(34) => msg.symbol = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.state = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GamePlayerDisconnected {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.symbol == String::default() { 0 } else { 1 + sizeof_len((&self.symbol).len()) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.state == server_events::PlayerInGameState::not_started { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.symbol != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.symbol))?; }
        if self.name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        if self.state != server_events::PlayerInGameState::not_started { w.write_with_tag(48, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GamePlayerReconnected {
    pub game_id: String,
    pub player_id: u32,
    pub state: PlayerInGameState,
}

impl<'a> MessageRead<'a> for GamePlayerReconnected {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
                Ok(24) => msg.state = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GamePlayerReconnected {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.state == server_events::PlayerInGameState::not_started { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.state != server_events::PlayerInGameState::not_started { w.write_with_tag(24, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}


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
    lobby_state = 0,
    player_status = 1,
    player_msg = 2,
    player_join_lobby = 3,
    player_leave_lobby = 4,
    lobby_game_updated = 5,
    player_join = 6,
    player_left = 7,
    player_disconnected = 8,
    player_reconnected = 9,
    game_start = 10,
    game_end = 11,
    game_player_move = 12,
}

impl Default for ServerMsgType {
    fn default() -> Self {
        ServerMsgType::lobby_state
    }
}

impl From<i32> for ServerMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => ServerMsgType::lobby_state,
            1 => ServerMsgType::player_status,
            2 => ServerMsgType::player_msg,
            3 => ServerMsgType::player_join_lobby,
            4 => ServerMsgType::player_leave_lobby,
            5 => ServerMsgType::lobby_game_updated,
            6 => ServerMsgType::player_join,
            7 => ServerMsgType::player_left,
            8 => ServerMsgType::player_disconnected,
            9 => ServerMsgType::player_reconnected,
            10 => ServerMsgType::game_start,
            11 => ServerMsgType::game_end,
            12 => ServerMsgType::game_player_move,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ServerMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "lobby_state" => ServerMsgType::lobby_state,
            "player_status" => ServerMsgType::player_status,
            "player_msg" => ServerMsgType::player_msg,
            "player_join_lobby" => ServerMsgType::player_join_lobby,
            "player_leave_lobby" => ServerMsgType::player_leave_lobby,
            "lobby_game_updated" => ServerMsgType::lobby_game_updated,
            "player_join" => ServerMsgType::player_join,
            "player_left" => ServerMsgType::player_left,
            "player_disconnected" => ServerMsgType::player_disconnected,
            "player_reconnected" => ServerMsgType::player_reconnected,
            "game_start" => ServerMsgType::game_start,
            "game_end" => ServerMsgType::game_end,
            "game_player_move" => ServerMsgType::game_player_move,
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
pub struct PlayerStatus {
    pub waiting_games: Vec<String>,
    pub ended_games: Vec<GameEnd>,
}

impl<'a> MessageRead<'a> for PlayerStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.waiting_games.push(r.read_string(bytes)?.to_owned()),
                Ok(34) => msg.ended_games.push(r.read_message::<GameEnd>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerStatus {
    fn get_size(&self) -> usize {
        0
        + self.waiting_games.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.ended_games.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.waiting_games { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.ended_games { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BoardState {
    pub game_id: String,
    pub player_in_turn: u32,
    pub players: Vec<game::Player>,
    pub cells: Vec<game::Cell>,
}

impl<'a> MessageRead<'a> for BoardState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_in_turn = r.read_uint32(bytes)?,
                Ok(26) => msg.players.push(r.read_message::<game::Player>(bytes)?),
                Ok(34) => msg.cells.push(r.read_message::<game::Cell>(bytes)?),
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
        + self.players.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.cells.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_in_turn != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_in_turn))?; }
        for s in &self.players { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.cells { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameEnd {
    pub game_id: String,
    pub result: game::GameStatus,
    pub winner_id: u32,
}

impl<'a> MessageRead<'a> for GameEnd {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.result = r.read_enum(bytes)?,
                Ok(24) => msg.winner_id = r.read_uint32(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.result != game::GameStatus::WAITING { w.write_with_tag(16, |w| w.write_enum(*&self.result as i32))?; }
        if self.winner_id != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.winner_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameMove {
    pub player_number: u32,
    pub player_id: u32,
    pub x: u32,
    pub y: u32,
}

impl<'a> MessageRead<'a> for GameMove {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_number = r.read_uint32(bytes)?,
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
                Ok(24) => msg.x = r.read_uint32(bytes)?,
                Ok(32) => msg.y = r.read_uint32(bytes)?,
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
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.x == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player_number != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.player_number))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.x != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.y))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GamePlayerDisconnected {
    pub game_id: String,
    pub player_id: u32,
    pub symbol: String,
    pub name: String,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.symbol != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.symbol))?; }
        if self.name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GamePlayerReconnected {
    pub game_id: String,
    pub player_id: u32,
}

impl<'a> MessageRead<'a> for GamePlayerReconnected {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        Ok(())
    }
}


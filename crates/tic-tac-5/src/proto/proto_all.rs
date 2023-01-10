// Automatically generated rust module for 'proto_all.proto' file

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
pub enum ClientMsgType {
    join_lobby = 0,
    lobby_msg = 1,
    create_lobby_game = 2,
    join_lobby_game = 3,
    leave_lobby_game = 4,
    player_move = 5,
    leave_game = 6,
}

impl Default for ClientMsgType {
    fn default() -> Self {
        ClientMsgType::join_lobby
    }
}

impl From<i32> for ClientMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => ClientMsgType::join_lobby,
            1 => ClientMsgType::lobby_msg,
            2 => ClientMsgType::create_lobby_game,
            3 => ClientMsgType::join_lobby_game,
            4 => ClientMsgType::leave_lobby_game,
            5 => ClientMsgType::player_move,
            6 => ClientMsgType::leave_game,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ClientMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "join_lobby" => ClientMsgType::join_lobby,
            "lobby_msg" => ClientMsgType::lobby_msg,
            "create_lobby_game" => ClientMsgType::create_lobby_game,
            "join_lobby_game" => ClientMsgType::join_lobby_game,
            "leave_lobby_game" => ClientMsgType::leave_lobby_game,
            "player_move" => ClientMsgType::player_move,
            "leave_game" => ClientMsgType::leave_game,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerMsgType {
    lobby_state = 0,
    player_msg = 1,
    player_join_lobby = 2,
    player_leave_lobby = 3,
    lobby_game_updated = 4,
    player_join = 5,
    game_start = 6,
    game_end = 7,
    player_left = 8,
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
            1 => ServerMsgType::player_msg,
            2 => ServerMsgType::player_join_lobby,
            3 => ServerMsgType::player_leave_lobby,
            4 => ServerMsgType::lobby_game_updated,
            5 => ServerMsgType::player_join,
            6 => ServerMsgType::game_start,
            7 => ServerMsgType::game_end,
            8 => ServerMsgType::player_left,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ServerMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "lobby_state" => ServerMsgType::lobby_state,
            "player_msg" => ServerMsgType::player_msg,
            "player_join_lobby" => ServerMsgType::player_join_lobby,
            "player_leave_lobby" => ServerMsgType::player_leave_lobby,
            "lobby_game_updated" => ServerMsgType::lobby_game_updated,
            "player_join" => ServerMsgType::player_join,
            "game_start" => ServerMsgType::game_start,
            "game_end" => ServerMsgType::game_end,
            "player_left" => ServerMsgType::player_left,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellType {
    EMPTY = 0,
    PLAYER_CELL = 1,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::EMPTY
    }
}

impl From<i32> for CellType {
    fn from(i: i32) -> Self {
        match i {
            0 => CellType::EMPTY,
            1 => CellType::PLAYER_CELL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CellType {
    fn from(s: &'a str) -> Self {
        match s {
            "EMPTY" => CellType::EMPTY,
            "PLAYER_CELL" => CellType::PLAYER_CELL,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    WAITING = 0,
    X_TURN = 1,
    O_TURN = 2,
    X_WON = 3,
    O_WON = 4,
    TIE = 5,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::WAITING
    }
}

impl From<i32> for GameStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => GameStatus::WAITING,
            1 => GameStatus::X_TURN,
            2 => GameStatus::O_TURN,
            3 => GameStatus::X_WON,
            4 => GameStatus::O_WON,
            5 => GameStatus::TIE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for GameStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "WAITING" => GameStatus::WAITING,
            "X_TURN" => GameStatus::X_TURN,
            "O_TURN" => GameStatus::O_TURN,
            "X_WON" => GameStatus::X_WON,
            "O_WON" => GameStatus::O_WON,
            "TIE" => GameStatus::TIE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Player {
    pub id: u32,
    pub socket_id: u32,
    pub player_number: u32,
    pub symbol: String,
    pub name: String,
    pub dead: bool,
}

impl<'a> MessageRead<'a> for Player {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(16) => msg.socket_id = r.read_uint32(bytes)?,
                Ok(24) => msg.player_number = r.read_uint32(bytes)?,
                Ok(34) => msg.symbol = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.dead = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Player {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.socket_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.socket_id) as u64) }
        + if self.player_number == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_number) as u64) }
        + if self.symbol == String::default() { 0 } else { 1 + sizeof_len((&self.symbol).len()) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.dead == false { 0 } else { 1 + sizeof_varint(*(&self.dead) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.socket_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.socket_id))?; }
        if self.player_number != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.player_number))?; }
        if self.symbol != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.symbol))?; }
        if self.name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        if self.dead != false { w.write_with_tag(48, |w| w.write_bool(*&self.dead))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub cell_type: proto_all::CellType,
    pub player: u32,
}

impl<'a> MessageRead<'a> for Cell {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.x = r.read_uint32(bytes)?,
                Ok(16) => msg.y = r.read_uint32(bytes)?,
                Ok(24) => msg.cell_type = r.read_enum(bytes)?,
                Ok(32) => msg.player = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Cell {
    fn get_size(&self) -> usize {
        0
        + if self.x == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
        + if self.cell_type == proto_all::CellType::EMPTY { 0 } else { 1 + sizeof_varint(*(&self.cell_type) as u64) }
        + if self.player == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.y))?; }
        if self.cell_type != proto_all::CellType::EMPTY { w.write_with_tag(24, |w| w.write_enum(*&self.cell_type as i32))?; }
        if self.player != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.player))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LobbyGame {
    pub game_id: String,
    pub players: u32,
    pub max_players: u32,
}

impl<'a> MessageRead<'a> for LobbyGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.players = r.read_uint32(bytes)?,
                Ok(24) => msg.max_players = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LobbyGame {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.players == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.players) as u64) }
        + if self.max_players == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.max_players) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.players != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.players))?; }
        if self.max_players != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.max_players))?; }
        Ok(())
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
    pub games: Vec<proto_all::LobbyGame>,
    pub players: Vec<proto_all::LobbyPlayer>,
}

impl<'a> MessageRead<'a> for LobbyState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.games.push(r.read_message::<proto_all::LobbyGame>(bytes)?),
                Ok(18) => msg.players.push(r.read_message::<proto_all::LobbyPlayer>(bytes)?),
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
pub struct GameStart {
    pub game_id: String,
    pub players: Vec<proto_all::Player>,
    pub cells: Vec<proto_all::Cell>,
}

impl<'a> MessageRead<'a> for GameStart {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.players.push(r.read_message::<proto_all::Player>(bytes)?),
                Ok(26) => msg.cells.push(r.read_message::<proto_all::Cell>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameStart {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + self.players.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.cells.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        for s in &self.players { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.cells { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameEnd {
    pub game_id: String,
    pub result: proto_all::GameStatus,
    pub winner: Option<proto_all::Player>,
}

impl<'a> MessageRead<'a> for GameEnd {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.result = r.read_enum(bytes)?,
                Ok(26) => msg.winner = Some(r.read_message::<proto_all::Player>(bytes)?),
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
        + if self.result == proto_all::GameStatus::WAITING { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + self.winner.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.result != proto_all::GameStatus::WAITING { w.write_with_tag(16, |w| w.write_enum(*&self.result as i32))?; }
        if let Some(ref s) = self.winner { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Move {
    pub player: u32,
    pub x: u32,
    pub y: u32,
}

impl<'a> MessageRead<'a> for Move {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player = r.read_uint32(bytes)?,
                Ok(16) => msg.x = r.read_uint32(bytes)?,
                Ok(24) => msg.y = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Move {
    fn get_size(&self) -> usize {
        0
        + if self.player == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player) as u64) }
        + if self.x == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.player))?; }
        if self.x != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.y))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameOptions {
    pub size: u32,
    pub players: u32,
}

impl<'a> MessageRead<'a> for GameOptions {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.size = r.read_uint32(bytes)?,
                Ok(16) => msg.players = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameOptions {
    fn get_size(&self) -> usize {
        0
        + if self.size == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + if self.players == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.players) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.size != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.size))?; }
        if self.players != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.players))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerJoinLobby {
    pub player_id: u32,
    pub name: String,
}

impl<'a> MessageRead<'a> for PlayerJoinLobby {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerJoinLobby {
    fn get_size(&self) -> usize {
        0
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.player_id))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerCreateGame {
    pub player_id: u32,
    pub name: String,
    pub preferred_symbol: String,
    pub options: Option<proto_all::GameOptions>,
}

impl<'a> MessageRead<'a> for PlayerCreateGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.preferred_symbol = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.options = Some(r.read_message::<proto_all::GameOptions>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerCreateGame {
    fn get_size(&self) -> usize {
        0
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.preferred_symbol == String::default() { 0 } else { 1 + sizeof_len((&self.preferred_symbol).len()) }
        + self.options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.player_id))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.preferred_symbol != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.preferred_symbol))?; }
        if let Some(ref s) = self.options { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerJoinGame {
    pub game_id: String,
    pub player_id: u32,
    pub name: String,
}

impl<'a> MessageRead<'a> for PlayerJoinGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerJoinGame {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.player_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.name != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerMove {
    pub game_id: String,
    pub player_number: u32,
    pub x: u32,
    pub y: u32,
}

impl<'a> MessageRead<'a> for PlayerMove {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_number = r.read_uint32(bytes)?,
                Ok(24) => msg.x = r.read_uint32(bytes)?,
                Ok(32) => msg.y = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerMove {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.player_number == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player_number) as u64) }
        + if self.x == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_number != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_number))?; }
        if self.x != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.y))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerLeave {
    pub game_id: String,
    pub player_id: u32,
}

impl<'a> MessageRead<'a> for PlayerLeave {
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

impl MessageWrite for PlayerLeave {
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


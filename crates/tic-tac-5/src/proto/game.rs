// Automatically generated rust module for 'game.proto' file

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
    pub ai: bool,
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
                Ok(64) => msg.ai = r.read_bool(bytes)?,
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
        + if self.ai == false { 0 } else { 1 + sizeof_varint(*(&self.ai) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.socket_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.socket_id))?; }
        if self.player_number != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.player_number))?; }
        if self.symbol != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.symbol))?; }
        if self.name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        if self.dead != false { w.write_with_tag(48, |w| w.write_bool(*&self.dead))?; }
        if self.ai != false { w.write_with_tag(64, |w| w.write_bool(*&self.ai))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub cell_type: CellType,
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
        + if self.cell_type == game::CellType::EMPTY { 0 } else { 1 + sizeof_varint(*(&self.cell_type) as u64) }
        + if self.player == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.player) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.y))?; }
        if self.cell_type != game::CellType::EMPTY { w.write_with_tag(24, |w| w.write_enum(*&self.cell_type as i32))?; }
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


// Automatically generated rust module for 'client_events.proto' file

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
    player_select_cell = 5,
    pause_game = 6,
    leave_game = 7,
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
            5 => ClientMsgType::player_select_cell,
            6 => ClientMsgType::pause_game,
            7 => ClientMsgType::leave_game,
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
            "player_select_cell" => ClientMsgType::player_select_cell,
            "pause_game" => ClientMsgType::pause_game,
            "leave_game" => ClientMsgType::leave_game,
            _ => Self::default(),
        }
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
    pub options: Option<GameOptions>,
}

impl<'a> MessageRead<'a> for PlayerCreateGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.preferred_symbol = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.options = Some(r.read_message::<GameOptions>(bytes)?),
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
    pub options: Option<GameOptions>,
}

impl<'a> MessageRead<'a> for PlayerJoinGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.player_id = r.read_uint32(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.options = Some(r.read_message::<GameOptions>(bytes)?),
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
        + self.options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.player_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.player_id))?; }
        if self.name != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.name))?; }
        if let Some(ref s) = self.options { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerSelectCell {
    pub game_id: String,
    pub x: u32,
    pub y: u32,
}

impl<'a> MessageRead<'a> for PlayerSelectCell {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.x = r.read_uint32(bytes)?,
                Ok(24) => msg.y = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerSelectCell {
    fn get_size(&self) -> usize {
        0
        + if self.game_id == String::default() { 0 } else { 1 + sizeof_len((&self.game_id).len()) }
        + if self.x == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.game_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.game_id))?; }
        if self.x != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.x))?; }
        if self.y != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.y))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerLeaveGame {
    pub game_id: String,
    pub player_id: u32,
}

impl<'a> MessageRead<'a> for PlayerLeaveGame {
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

impl MessageWrite for PlayerLeaveGame {
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


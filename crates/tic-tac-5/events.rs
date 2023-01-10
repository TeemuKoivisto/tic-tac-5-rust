#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerJoin {
    pub game_id: String,
    pub player_id: u32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerMove {
    pub game_id: String,
    pub player_number: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerLeave {
    pub game_id: String,
    pub player_number: u32,
}

#[derive(Debug, Clone)]
pub struct PlayerJoinWithClientId {
    pub client_id: u32,
    pub player_join: PlayerJoin,
}

#[derive(Debug)]
pub enum GameEvent {
    JoinGame(PlayerJoinWithClientId),
    PlayerQuitOrDisconnected(PlayerLeave),
    Move(PlayerMove),
    Tick(),
}

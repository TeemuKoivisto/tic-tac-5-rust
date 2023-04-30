use tic_tac_5::proto::proto_all::*;

pub struct Lobby {
    pub lobby_players: Vec<LobbyPlayer>,
    pub lobby_chat: Vec<String>,
    // Use this and trait Broadcastable to send messages either to game or lobby depending who has the connection?
    // problem -> lot of moving of connections
    // plus -> no need to loop players in games when broadcasting lobby state
    // pub connections: Vec<Arc<Mutex<Connection>>>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            lobby_players: Vec::new(),
            lobby_chat: Vec::new(),
        }
    }

    pub fn player_join_lobby(&mut self, data: PlayerJoinLobby) {
        self.lobby_players.insert(
            0,
            LobbyPlayer {
                player_id: data.player_id,
                name: data.name,
            },
        );
    }

    pub fn player_leave_lobby(&mut self, player_id: u32) {
        self.lobby_players.retain(|p| p.player_id != player_id);
    }
}

use axum::extract::ws::WebSocket;
use tokio::sync::broadcast::{self, error::SendError};

use crate::state::events::{
    ClientConnected, ClientToLobbyEvent, GameToClientEvent, LobbyToClientEvent,
};

use super::session::Session;

pub struct SessionHandle {
    pub socket_id: u32,
    pub actor: Session,
    pub client_sender: broadcast::Sender<ClientToLobbyEvent>,
    pub client_receiver: broadcast::Receiver<ClientToLobbyEvent>,
    pub lobby_sender: broadcast::Sender<LobbyToClientEvent>,
    pub game_sender: broadcast::Sender<GameToClientEvent>,
}

impl SessionHandle {
    pub fn new(socket: WebSocket, socket_id: u32) -> Self {
        let (client_sender, client_receiver) = broadcast::channel(64);
        let (lobby_sender, lobby_receiver) = broadcast::channel(64);
        let (game_sender, game_receiver) = broadcast::channel(64);
        let actor = Session::new(
            socket_id,
            socket,
            client_sender.clone(),
            lobby_receiver,
            game_sender.clone(),
            game_receiver,
        );
        let socket_id = actor.socket_id;
        Self {
            actor,
            socket_id,
            client_sender,
            client_receiver,
            lobby_sender,
            game_sender,
        }
    }

    pub async fn restore(&mut self, socket: WebSocket) {
        self.actor.restore(socket).await;
    }

    pub fn subscribe(
        &self,
        sender: &broadcast::Sender<ClientToLobbyEvent>,
        player_id: u32,
    ) -> Result<usize, SendError<ClientToLobbyEvent>> {
        sender.send(ClientToLobbyEvent::Connected(ClientConnected {
            socket_id: self.socket_id,
            player_id,
            waiting_game: self.actor.state.waiting_game.clone(),
            subscribed_games: self.actor.state.get_game_ids(),
            lobby_sender: self.lobby_sender.clone(),
            game_sender: self.game_sender.clone(),
        }))
    }
}

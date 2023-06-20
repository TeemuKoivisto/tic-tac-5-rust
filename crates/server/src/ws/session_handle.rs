use axum::extract::ws::WebSocket;
use tokio::sync::broadcast::{self, error::SendError};

use crate::state::events::{ClientToLobbyEvent, GameToClientEvent, LobbyToClientEvent};

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

    pub fn subscribe(
        &self,
        sender: &broadcast::Sender<ClientToLobbyEvent>,
    ) -> Result<usize, SendError<ClientToLobbyEvent>> {
        sender.send(ClientToLobbyEvent::Connected(
            self.socket_id,
            self.lobby_sender.clone(),
            self.game_sender.clone(),
        ))
    }
}

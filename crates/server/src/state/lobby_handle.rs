use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;

use super::{
    events::{ClientToLobbyEvent, LobbyToClientEvent},
    lobby::{run_lobby, Lobby},
};

pub struct LobbyHandle {
    pub client_sender: broadcast::Sender<ClientToLobbyEvent>,
    pub lobby_receiver: broadcast::Receiver<LobbyToClientEvent>,
}

impl LobbyHandle {
    pub fn new() -> Self {
        let (client_sender, client_receiver) = broadcast::channel(64);
        let (lobby_sender, lobby_receiver) = broadcast::channel(64);
        let lobby = Lobby::new(lobby_sender, client_receiver);
        run_lobby(lobby);
        Self {
            client_sender,
            lobby_receiver,
        }
    }

    pub fn subscribe(
        &self,
        sender: &broadcast::Sender<LobbyToClientEvent>,
    ) -> Result<usize, SendError<LobbyToClientEvent>> {
        sender.send(LobbyToClientEvent::Subscribe(self.client_sender.clone()))
    }
}

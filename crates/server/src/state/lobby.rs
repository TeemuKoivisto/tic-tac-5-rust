use tic_tac_5::proto::proto_all::*;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;

use super::{
    events::{ClientEvent, LobbyEvent},
    lobby_actor::{run_lobby, LobbyActor},
};

pub struct Lobby {
    pub client_sender: broadcast::Sender<ClientEvent>,
    pub lobby_receiver: broadcast::Receiver<LobbyEvent>,
    // Use this and trait Broadcastable to send messages either to game or lobby depending who has the connection?
    // problem -> lot of moving of connections
    // plus -> no need to loop players in games when broadcasting lobby state
    // pub connections: Vec<Arc<Mutex<Connection>>>,
}

impl Lobby {
    pub fn new() -> Self {
        let (client_sender, client_receiver) = broadcast::channel(64);
        let (lobby_sender, lobby_receiver) = broadcast::channel(64);
        let lobby = LobbyActor::new(lobby_sender, client_receiver);
        run_lobby(lobby);
        Self {
            client_sender,
            lobby_receiver,
        }
    }

    pub fn subscribe(
        &self,
        sender: &broadcast::Sender<LobbyEvent>,
    ) -> Result<usize, SendError<LobbyEvent>> {
        sender.send(LobbyEvent::Subscribe(self.client_sender.clone()))
    }
}
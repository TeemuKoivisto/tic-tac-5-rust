use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, error::SendError};
use tokio_tungstenite::WebSocketStream;

use crate::state::events::{ClientEvent, GameEvent, LobbyEvent};

use super::ws_session::WsSession;

pub struct WsSessionHandle {
    pub socket_id: u32,
    pub actor: WsSession,
    pub client_sender: broadcast::Sender<ClientEvent>,
    pub client_receiver: broadcast::Receiver<ClientEvent>,
    pub lobby_sender: broadcast::Sender<LobbyEvent>,
    pub game_sender: broadcast::Sender<GameEvent>,
}

impl WsSessionHandle {
    pub fn new(socket: WebSocketStream<TcpStream>, socket_id: u32) -> Self {
        let (client_sender, client_receiver) = broadcast::channel(64);
        let (lobby_sender, lobby_receiver) = broadcast::channel(64);
        let (game_sender, game_receiver) = broadcast::channel(64);
        let actor = WsSession::new(
            socket_id,
            socket,
            client_sender.clone(),
            lobby_receiver,
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
        sender: &broadcast::Sender<ClientEvent>,
    ) -> Result<usize, SendError<ClientEvent>> {
        sender.send(ClientEvent::Connected(
            self.socket_id,
            self.lobby_sender.clone(),
            self.game_sender.clone(),
        ))
    }
}

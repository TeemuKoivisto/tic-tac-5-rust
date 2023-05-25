use tic_tac_5::proto::proto_all::*;
use tokio::sync::broadcast::error::SendError;
use tokio::{sync::broadcast, task::JoinHandle};
use uuid::Uuid;

use crate::state::events::{ClientEvent, GameEvent};

use super::game::Game;
use super::listed_game::ListedGame;

pub struct GameHandle {
    pub id: Uuid,
    pub client_sender: broadcast::Sender<ClientEvent>,
    pub game_receiver: broadcast::Receiver<GameEvent>,
    // Use this and trait Broadcastable to send messages either to game or lobby depending who has the connection?
    // problem -> lot of moving of connections
    // plus -> no need to loop players in games when broadcasting lobby state
    // pub connections: Vec<Arc<Mutex<Connection>>>,
}

impl GameHandle {
    pub fn new(lobby_game: &ListedGame) -> Self {
        let (client_sender, client_receiver) = broadcast::channel(64);
        let (game_sender, game_receiver) = broadcast::channel(64);
        let actor = Game::new(lobby_game, None, game_sender, client_receiver);
        let id = actor.id;
        run_game(actor);
        Self {
            id,
            client_sender,
            game_receiver,
        }
    }

    pub fn subscribe(
        &self,
        sender: &broadcast::Sender<GameEvent>,
    ) -> Result<usize, SendError<GameEvent>> {
        sender.send(GameEvent::Subscribe(
            self.id.to_string(),
            self.client_sender.clone(),
        ))
    }
}

pub fn run_game(mut actor: Game) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(ev) = actor.client_receiver.recv() => {
                    actor.handle_client_event(ev).await;
                },
                else => {
                    break;
                },
            }
        }
    })
}

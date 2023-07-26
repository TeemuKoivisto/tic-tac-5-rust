use tokio::{sync::broadcast, task::JoinHandle};
use uuid::Uuid;

use crate::state::events::{ClientToGameEvent, GameToClientEvent, GameToLobbyEvent};

use super::game::Game;
use super::listed_game::ListedGame;

pub struct GameHandle {
    pub id: Uuid,
    pub client_sender: broadcast::Sender<ClientToGameEvent>,
    pub game_receiver: broadcast::Receiver<GameToClientEvent>,
}

impl GameHandle {
    pub fn new(
        lobby_game: &ListedGame,
        game_to_lobby_sender: broadcast::Sender<GameToLobbyEvent>,
    ) -> Self {
        let (client_sender, client_receiver) = broadcast::channel(64);
        let (game_sender, game_receiver) = broadcast::channel(64);
        let actor = Game::new(
            lobby_game,
            None,
            game_sender,
            game_to_lobby_sender,
            client_receiver,
        );
        let id = actor.id;
        run_game(actor);
        Self {
            id,
            client_sender,
            game_receiver,
        }
    }

    // pub fn subscribe(
    //     &self,
    //     sender: &broadcast::Sender<GameToClientEvent>,
    // ) -> Result<usize, SendError<GameToClientEvent>> {
    //     sender.send(GameToClientEvent::Subscribe(
    //         self.id.to_string(),
    //         self.client_sender.clone(),
    //     ))
    // }
}

pub fn run_game(mut actor: Game) -> JoinHandle<()> {
    tokio::spawn(async move {
        let dur = std::time::Duration::from_secs_f64(2.0);
        let mut interval = tokio::time::interval(dur);
        loop {
            tokio::select! {
                Ok(ev) = actor.client_receiver.recv() => {
                    actor.handle_client_event(ev).await;
                },
                _ = interval.tick() => {
                    // println!("tick");
                    if !actor.check_if_running() {
                        break;
                    }
                },
                else => {
                    break;
                },
            }
        }
        actor.send_end_game();
    })
}

use crate::connection::ConnectionManager;
use log::debug;
use quick_protobuf::{MessageWrite, Writer};
use std::sync::Arc;
use tic_tac_5::{events::ServerEvent, proto::proto_all::*};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

pub async fn write_server_msg(
    event: ServerEvent,
    conn_manager_mutex: Arc<Mutex<ConnectionManager>>,
) {
    let mut conn_manager = conn_manager_mutex.lock().await;
    match event {
        ServerEvent::ClientConnected(_client_id) => {
            // debug!("ServerEvents::ClientConnected {:?}", conn);
            // conn_manager.add(conn);
        }
        ServerEvent::ClientDisconnected(client_id) => {
            debug!("ServerEvents::ClientDisconnected");
            conn_manager.remove(client_id);
        }
        ServerEvent::LobbyGames(_) => {}
        ServerEvent::PlayerMsg(_payload) => {}
        ServerEvent::PlayerJoinLobby(_payload) => {}
        ServerEvent::PlayerCreateGame(_payload) => {}
        ServerEvent::PlayerLeaveLobby(_payload) => {}
        ServerEvent::LobbyGameUpdated(_payload) => {}
        ServerEvent::PlayerJoin(_payload) => {}
        ServerEvent::GameStart(start) => {
            debug!("ServerEvents::GameStart");
            conn_manager
                .broadcast(
                    serialize_server_event(ServerMsgType::game_start, &start),
                    start.game_id,
                )
                .await;
        }
        ServerEvent::GameEnd(payload) => {
            debug!("ServerEvents::GameEnd");
            conn_manager
                .broadcast(
                    serialize_server_event(ServerMsgType::game_end, &payload),
                    payload.game_id,
                )
                .await;
        }
        // ServerEvent::Tick(state) => {
        //     conn_manager
        //         .broadcast(
        //             serialize_server_event(ServerMsgType::tick, &state),
        //             state.game_id,
        //         )
        //         .await;
        // }
        ServerEvent::Quit(payload) => {
            debug!("ServerEvents::Quit");
            conn_manager
                .broadcast(
                    serialize_server_event(ServerMsgType::player_left, &payload),
                    payload.game_id,
                )
                .await;
        }
    }
}

// Serialize a server event and a header byte into bytes to send through websocket.
pub fn serialize_server_event<M: MessageWrite>(header: ServerMsgType, payload: &M) -> Message {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);
    writer.write_u8(header.try_into().unwrap()).unwrap();
    payload
        .write_message(&mut writer)
        .expect("Unable to serialize message!");
    Message::Binary(out)
}

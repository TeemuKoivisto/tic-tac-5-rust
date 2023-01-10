use log::{debug, error};
use quick_protobuf::{BytesReader, MessageRead};

use crate::connection::ConnectionManager;
use crate::context::Context;
use crate::game::write_server_msg::{serialize_server_event, write_server_msg};
use crate::Connection;
use futures_util::stream::StreamExt;
use std::sync::Arc;
use tic_tac_5::{
    events::{GameEvent, ServerEvent},
    proto::proto_all::*,
};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;

// Broadcast all the incoming game state to the clients.
pub async fn broadcast(
    conn_manager: Arc<Mutex<ConnectionManager>>,
    mut rx: UnboundedReceiver<ServerEvent>,
) {
    while let Some(event) = rx.recv().await {
        write_server_msg(event, conn_manager.clone()).await;
    }
}

// Listen for incoming data from clients.
pub async fn listen(ctx: Arc<Context>, ws_stream: WebSocketStream<TcpStream>, socket_id: u32) {
    let (sender, mut receiver) = ws_stream.split();
    ctx.handle_player_connect(socket_id, sender).await;

    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                let mut msg = msg.into_data();
                let message_type = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                // debug!("New message {}", message_type);
                match ClientMsgType::try_from(message_type) {
                    Ok(ClientMsgType::join_lobby) => {
                        if let Ok(player_join) = PlayerJoinLobby::from_reader(&mut reader, &msg) {
                            debug!("ClientMsgType::join_lobby {:#?}", player_join);
                            ctx.handle_join_lobby(socket_id, player_join).await;
                        }
                    }
                    Ok(ClientMsgType::lobby_msg) => {}
                    Ok(ClientMsgType::create_lobby_game) => {}
                    Ok(ClientMsgType::join_lobby_game) => {}
                    Ok(ClientMsgType::player_move) => {}
                    _ => error!("Unknown header: {}", message_type),
                }
            } else if msg.is_close() {
                break; // When we break, we disconnect.
            }
        } else {
            break; // When we break, we disconnect.
        }
    }

    // If we reach here, it means the client quit or disconnected. Send quit event to each room the client was in.
    ctx.handle_player_disconnect(socket_id).await;
    // ctx.broadcast_lobby_state().await;
}

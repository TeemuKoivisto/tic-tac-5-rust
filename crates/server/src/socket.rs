use log::{debug, error};
use quick_protobuf::{BytesReader, MessageRead};
use uuid::Uuid;

use crate::{connection::Connection, context::Context, player_context::PlayerContext};
use futures_util::stream::StreamExt;
use std::sync::Arc;
use tic_tac_5::proto::proto_all::*;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::WebSocketStream;

pub async fn listen(ctx: Arc<Context>, ws_stream: WebSocketStream<TcpStream>, socket_id: u32) {
    let (sender, mut receiver) = ws_stream.split();
    let conn = Arc::new(Mutex::new(Connection::new(socket_id, sender)));
    let mut player = PlayerContext::new(socket_id, conn.clone());
    ctx.player_connect(socket_id, conn.clone()).await;

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
                            ctx.join_lobby(socket_id, player_join).await;
                            ctx.broadcast_lobby_state().await;
                        }
                    }
                    Ok(ClientMsgType::create_lobby_game) => {
                        if let Ok(create_game) = PlayerCreateGame::from_reader(&mut reader, &msg) {
                            debug!("ClientMsgType::create_lobby_game {:#?}", create_game);
                            let (started, game_mut) =
                                ctx.create_lobby_game(socket_id, create_game).await;
                            game_mut
                                .lock()
                                .await
                                .add_player_connection(socket_id, conn.clone());
                            player.set_joined_game(game_mut.clone()).await;
                            if started {
                                ctx.start_game(game_mut).await;
                            }
                            ctx.broadcast_lobby_state().await;
                        }
                    }
                    Ok(ClientMsgType::join_lobby_game) => {
                        if let Ok(player_join) = PlayerJoinGame::from_reader(&mut reader, &msg) {
                            debug!("ClientMsgType::join_lobby_game {:#?}", player_join);
                            let (started, game_mut) =
                                ctx.join_lobby_game(socket_id, player_join).await;
                            game_mut
                                .lock()
                                .await
                                .add_player_connection(socket_id, conn.clone());
                            player.set_joined_game(game_mut.clone()).await;
                            if started {
                                ctx.start_game(game_mut).await;
                            }
                            ctx.broadcast_lobby_state().await;
                        }
                    }
                    Ok(ClientMsgType::player_select_cell) => {
                        if let Ok(payload) = PlayerSelectCell::from_reader(&mut reader, &msg) {
                            debug!("ClientMsgType::player_select_cell {:#?}", payload);
                            let ended = player.player_select_cell(&payload).await;
                            if ended {
                                player.remove_joined_game().await;
                                let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                                ctx.end_game(game_id).await;
                                ctx.remove_game(game_id).await;
                                ctx.broadcast_lobby_state().await;
                            }
                        }
                    }
                    Ok(ClientMsgType::leave_game) => {
                        if let Ok(payload) = PlayerLeaveGame::from_reader(&mut reader, &msg) {
                            debug!("ClientMsgType::player_leave {:#?}", payload);
                            let game_id = Uuid::parse_str(&payload.game_id).unwrap();
                            let ended = ctx.player_leave_game(socket_id, payload).await;
                            if ended {
                                player.remove_joined_game().await;
                                ctx.end_game(game_id).await;
                                ctx.remove_game(game_id).await;
                            }
                            ctx.broadcast_lobby_state().await;
                        }
                    }
                    _ => error!("Unknown header: {}", message_type),
                }
            } else if msg.is_close() {
                break; // When we break, we disconnect.
            }
        } else {
            break; // When we break, we disconnect.
        }
    }
    // We must first delete the connection from conn_manager in order to not to deadlock sending of disconnect event
    ctx.remove_player_connection(conn.clone()).await;
    ctx.player_disconnect(conn).await;
    ctx.broadcast_lobby_state().await;
}

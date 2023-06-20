use axum::extract::ws::Message;
use quick_protobuf::{MessageWrite, Writer};
use tic_tac_5::proto::proto_all::*;

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

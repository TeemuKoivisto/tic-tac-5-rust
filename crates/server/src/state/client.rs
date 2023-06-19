#[derive(Debug, Clone)]
pub struct Client {
    pub player_id: u32,
    pub name: String,
    pub socket_id: u32,
}

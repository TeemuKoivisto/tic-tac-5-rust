use std::collections::HashMap;

pub struct SessionManager {
    sessions: HashMap<String, String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn insert_session(&mut self, user_id: &str, socket_id: u32) {
        self.sessions
            .insert(user_id.to_string(), user_id.to_string());
    }

    pub fn delete_session(&mut self, user_id: &str) {
        self.sessions.remove(&user_id.to_string());
    }
}

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TicTac5Token {
    pub exp: u64,
    pub iat: u64,
    pub player_id: u32,
}

pub struct JwtManager {
    sessions: HashMap<String, TicTac5Token>,
    jwt_secret: Hmac<Sha512>,
}

#[derive(Debug)]
pub enum JwtError {
    NoSession(TicTac5Token),
    Expired,
    Other(jwt::Error),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            JwtError::NoSession(ref x) => write!(f, "No current session with jwt found"),
            JwtError::Expired => write!(f, "JWT session expired"),
            JwtError::Other(ref x) => write!(f, "{}", x),
        }
    }
}

impl std::error::Error for JwtError {}

impl JwtManager {
    pub fn new(jwt_secret: &String) -> Self {
        Self {
            sessions: HashMap::new(),
            jwt_secret: Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap(),
        }
    }

    pub fn insert_session(&mut self, jwt: String, token: &TicTac5Token) {
        self.sessions.insert(jwt.to_string(), token.clone());
    }

    pub fn delete_session(&mut self, jwt: &str) {
        self.sessions.remove(&jwt.to_string());
    }

    pub fn encode_login(&mut self, player_id: u32) -> (String, u64) {
        let time = chrono::Utc::now().timestamp_millis() as u64 / 1000;
        let exp = time + 60 * 60 * 24 * 14; // two weeks in seconds
        let claims = TicTac5Token {
            iat: time,
            exp,
            player_id,
        };
        let cref = &claims;
        let token = &cref.sign_with_key(&self.jwt_secret).unwrap();
        self.sessions.insert(token.to_string(), claims);
        (token.to_string(), exp)
    }

    pub fn decode(&self, jwt: &str) -> Result<TicTac5Token, JwtError> {
        let decrypted: Result<TicTac5Token, jwt::Error> = jwt.verify_with_key(&self.jwt_secret);
        if decrypted.is_err() {
            return Err(JwtError::Other(decrypted.unwrap_err()));
        }
        let token = decrypted.unwrap();
        // TODO does verify_with_key check exp already?
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if token.exp < now.as_secs() {
            return Err(JwtError::Expired);
        } else if !self.sessions.contains_key(jwt) {
            return Err(JwtError::NoSession(token));
        }
        return Ok(token);
    }
}

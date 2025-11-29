use crate::auth::User;
use rand::{Rng, distr::Alphanumeric, rng};
use std::collections::HashMap;

/// TODO: Make all of this private and make public accessors
pub struct TokenStore {
    tokens: HashMap<String, User>,
}

impl Default for TokenStore {
    fn default() -> Self {
        Self::new()
    }
}
impl TokenStore {
    pub fn new() -> Self {
        TokenStore {
            tokens: HashMap::new(),
        }
    }

    pub fn generate_token(&mut self, user: User) -> String {
        let token: String = rng()
            .sample_iter(Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        self.tokens.insert(token.clone(), user);
        token
    }

    pub fn remove_token(&mut self, token: &str) {
        self.tokens.remove(token);
    }

    pub fn is_valid(&self, token: &str) -> bool {
        self.tokens.contains_key(token)
    }

    pub fn get_user(&self, token: &str) -> Option<&User> {
        self.tokens.get(token)
    }
}

use std::fmt;
use super::token_types;

pub struct Token {
    token_type: token_types::TokenTypes,
    value: Option<String>,
}

impl Token {
    pub fn new(token_type: token_types::TokenTypes, value: Option<String>) -> Token {
        Token {
            token_type,
            value,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Token")
            .field("token_type", &self.token_type)
            .field("value", &self.value)
            .finish()
    }
}

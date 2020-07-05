pub(super) struct Token {
    pub ttype: super::token_types::TokenTypes,
    pub value: Option<String>,
}

impl Token {
    #[allow(dead_code)]
    pub fn new() -> Token {
        Token {
            ttype: super::token_types::TokenTypes::IDENTIFIER,
            value: None,
        }
    }
}

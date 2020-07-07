use std::fmt;
use super::token;

#[derive(Debug)]
pub enum Metatype {
    TEXT,
    CODE,
    ECHO,
    COMMENT,
}

pub struct Metadata {
    mtype: Metatype,
    data: String,
    tokens: Option<Vec<token::Token>>,
}

impl Metadata {
    pub fn new(t: Metatype, d: String, tks: Option<Vec<token::Token>>) -> Metadata {
        Metadata {
            mtype: t,
            data: d,
            tokens: tks,
        }
    }
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Metadata")
            .field("mtype", &self.mtype)
            .field("data", &self.data)
            .field("tokens", &self.tokens)
            .finish()
    }
}

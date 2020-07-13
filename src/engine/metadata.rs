use std::fmt;
use super::token;
use super::token_types;

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

    pub fn get_tokens(&self) -> &Option<Vec<token::Token>> {
        &self.tokens
    }

    pub fn get_type(&self) -> &Metatype {
        &self.mtype
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }
}

pub struct Tokenator<'a> {
    buffer: &'a Vec<token::Token>,
    index: usize,
}

impl<'a> Tokenator<'a> {
    pub fn new(tokens: &Vec<token::Token>) -> Tokenator {
        Tokenator {
            buffer: tokens,
            index: 0,
        }
    }
}

pub trait TokenatorTrait {
    fn next(&mut self)   -> Option<&token::Token>;
    fn look(&self)       -> Option<&token::Token>;
    fn look_back(&self)  -> Option<&token::Token>;
    fn look_ahead(&self) -> Option<&token::Token>;
    fn skip_all(&mut self);
    fn match_next(&mut self, token_type: token_types::TokenTypes) -> bool;
}

impl<'a> TokenatorTrait for Tokenator<'a> {
    fn next(&mut self) -> Option<&token::Token> {
        if self.buffer.get(self.index).is_none() {
            None
        }
        else {
            let tmp = self.buffer.get(self.index);
            self.index += 1;
            tmp
        }
    }

    fn look(&self) -> Option<&token::Token> {
        self.buffer.get(self.index)
    }

    fn look_back(&self) -> Option<&token::Token> {
        self.buffer.get(self.index - 1)
    }

    fn look_ahead(&self) -> Option<&token::Token> {
        self.buffer.get(self.index + 1)
    }

    fn skip_all(&mut self) {
        self.index = self.buffer.len();
    }

    fn match_next(&mut self, token_type: token_types::TokenTypes) -> bool {
        match self.buffer.get(self.index) {
            Some(tk) => {
                if tk.get_type() == token_type {
                    self.next();
                    true
                }
                else {
                    false
                }
            },
            None => false,
        }
    }
}

pub type Metainfo = Vec<Metadata>;

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Metadata")
            .field("mtype", &self.mtype)
            .field("data", &self.data)
            .field("tokens", &self.tokens)
            .finish()
    }
}

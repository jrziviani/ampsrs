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
}

pub struct Tokenable<I: Iterator> {
    iter: I,
    data: Option<Option<I::Item>>,
}

impl<I: Iterator> Tokenable<I> {
    pub fn new(iter: I) -> Tokenable<I> {
        Tokenable { iter, data: None }
    }
}

impl<I: Iterator> Iterator for Tokenable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.data.take() {
            Some(v) => v,
            None => self.iter.next(),
        }
    }
}

impl<I: Iterator> Tokenable<I> {
    pub fn look(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.data.get_or_insert_with(|| iter.next()).as_ref()
    }
}

// pub trait TokenableIterator : Iterator {
//     fn look(&mut self) -> Option<Self::Item>;
// }
//  
// impl<I: Iterator> TokenableIterator for Tokenable<I> {
//     fn look(&mut self) -> Option<Self::Item> {
//         match self.data.take() {
//             Some(v) => Some(v),
//             None => None,
//         }
//     }
// }

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

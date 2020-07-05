use std::fmt;

#[derive(Debug)]
pub(super) enum Metatype {
    TEXT,
    CODE,
    ECHO,
    COMMENT,
}

#[allow(dead_code)]
pub(super) struct Metarange {
    start: u64,
    end: u64,
    line: u64,
}

#[allow(dead_code)]
pub(super) struct Metadata {
    range: Metarange,
    mtype: Metatype,
    data: String,
    tokens: Vec<super::token::Token>,
}

impl Metadata {
    pub fn new(t: Metatype, d: String) -> Metadata {
        Metadata {
            range: Metarange {
                start: 0,
                end: 0,
                line: 0,
            },
            mtype: t,
            data: d,
            tokens: Vec::new(),
        }
    }

    pub fn add_token(&mut self, t: super::token::Token) {
        self.tokens.push(t);
    }
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Metadata")
            .field("mtype", &self.mtype)
            .field("data", &self.data)
            .finish()
    }
}

#[allow(dead_code)]
pub(super) struct Metainfo {
    metadata: Vec<Metadata>,
}

impl Metainfo {
    pub fn new() -> Metainfo {
        Metainfo {
            metadata: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_metadata(&mut self, d: Metadata) {
        self.metadata.push(d);
    }
}

impl fmt::Debug for Metainfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Metainfo")
            .field("Metadata", &self.metadata)
            .finish()
    }
}

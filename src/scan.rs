use regex::Regex;

pub trait PeekableIterator : std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

#[allow(dead_code)]
pub struct Scan {
    metainfo: super::metadata::Metainfo,
}

impl Scan {
    pub fn new() -> Scan {
        Scan {
            metainfo: super::metadata::Metainfo::new(),
        }
    }

    pub fn print_block(&mut self) {
        println!("{:?}", self.metainfo);
    }

    pub fn parse_block(&mut self, line: String) {
        let re_block = Regex::new(r#"(?P<code>\{% [a-z][a-zA-Z0-9\-,.%_\\\[\]"() ]+ %\})|(?P<echo>\{= [a-z][a-zA-Z0-9_\[\]"]+ =\})|(?P<text>.[^\{]*)"#).unwrap();

        for group in re_block.captures_iter(&line) {
            for name in re_block.capture_names() {
                match name {
                    Some(s) => match group.name(s) {
                        Some(gs) => {
                            let mtype : super::metadata::Metatype;
                            if s == "text" {
                                mtype = super::metadata::Metatype::TEXT;
                            }
                            else if s == "code" {
                                mtype = super::metadata::Metatype::CODE;
                            }
                            else if s == "echo" {
                                mtype = super::metadata::Metatype::ECHO;
                            }
                            else {
                                mtype = super::metadata::Metatype::COMMENT;
                            }

                            self.metainfo.add_metadata(super::metadata::Metadata::new(
                                    mtype,
                                    String::from(gs.as_str()))
                            );
                        }
                        None => continue,
                    }
                    None => continue,
                }
            }
        }
    }

    fn parse_string<P>(&mut self, meta: &mut super::metadata::Metadata, iter: &mut P)
    where P: PeekableIterator<Item=char> {
        let mut data: String = String::new();

        assert!(iter.peek() == Some(&'"'));
        iter.next();

        loop {
            match iter.peek() {
                None => break,
                Some(&ch) => {
                    match ch {
                        '"' => break,
                        _ => {
                            data.push(ch);
                        }
                    }
                }
            }
            iter.next();
        }

        meta.add_token(super::token::Token {
            ttype: super::token_types::TokenTypes::STRING,
            value: Some(data),
        });
    }

    fn parse_number<P>(&mut self, meta: &mut super::metadata::Metadata, iter: &mut P)
    where P: PeekableIterator<Item=char> {
        let mut data: u64 = 0;

        loop {
            match iter.peek() {
                None => break,
                Some(&ch) => {
                    match ch {
                        '0'..='9' => {
                            let digit = ch as u64 - '0' as u64;
                            data = data * 10 + digit;
                        }
                        _ => break,
                    }
                }
            }
            iter.next();
        }

        meta.add_token(super::token::Token {
            ttype: super::token_types::TokenTypes::STRING,
            value: Some(data.to_string()),
        });
    }

    fn parse_id<P>(&mut self, meta: &mut super::metadata::Metadata, iter: &mut P)
    where P: PeekableIterator<Item=char> {
        let mut data: String = String::new();

        loop {
            match iter.peek() {
                None => break,
                Some(&ch) => {
                    match ch {
                        'a'..='z' | 'A'..='Z' | '_' => {
                            data.push(ch);
                        }
                        _ => break,
                    }
                }
            }
            iter.next();
        }

        println!("{:?}", data);

        meta.add_token(super::token::Token {
            ttype: super::token_types::TokenTypes::STRING,
            value: Some(data),
        });
    }

    pub fn scan(&mut self, line: String) {
        let mut s = String::new();
        let mut meta = super::metadata::Metadata::new(super::metadata::Metatype::TEXT, s);
        let mut iter = line.chars().peekable();
        loop {
            match iter.peek() {
                None => break,
                Some(&ch) => {
                    match ch {
                        // skip empty spaces
                        ' ' | '\t' | '\r' => {
                            iter.next();
                        }
                        // strings starts with "
                        '"' => {
                            self.parse_string(&mut meta, &mut iter);
                            iter.next();
                        }
                        // digits
                        '0'..='9' => {
                            self.parse_number(&mut meta, &mut iter);
                            iter.next();
                        }
                        // identifiers
                        'a'..='z' => {
                            self.parse_id(&mut meta, &mut iter);
                            iter.next();
                        }
                        // ids and errors
                        _ => {
                            //eprintln!("Inexpected {}", ch);
                            iter.next();
                        }
                    }
                }
            }
        }
    }
}

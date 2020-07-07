pub mod scanner {
    use regex::Regex;
    use std::io::BufRead;

    use crate::metadata;
    use crate::token_types;
    use crate::token;

    pub trait PeekableIterator : std::iter::Iterator {
        fn peek(&mut self) -> Option<&Self::Item>;
    }

    impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
        fn peek(&mut self) -> Option<&Self::Item> {
            std::iter::Peekable::peek(self)
        }
    }

    // implements the regular expression responsible to look for {= .* =} and
    // {% .* %}. Each of these two blocks will be evaluated, anything else is
    // just text.
    const REG_BLOCK: &str = concat!(r#"(?P<code>\{% [a-z][a-zA-Z0-9\-,.%_\\\[\]"() ]+ %\})|"#,
                                     r#"(?P<echo>\{= [a-z][a-zA-Z0-9_\[\]"]+ =\})|"#,
                                     r#"(?P<text>.[^\{]*)"#);

    const REG_INNER_BLOCK: &str = r"^\{[%|=] (?P<code>.+) [%|=]\}";

    pub fn scan(file: &mut std::io::BufReader<std::fs::File>) -> Vec<metadata::Metadata> {
        let mut ret: Vec<metadata::Metadata> = Vec::new();

        for line in file.lines() {
            let mut data = parse_block(&line.unwrap());
            ret.append(&mut data);
        }

        ret
    }

    fn parse_block(line: &String) -> Vec<metadata::Metadata> {
        let re_block = Regex::new(REG_BLOCK).unwrap();
        let mut ret: Vec<metadata::Metadata> = Vec::new();

        for group in re_block.captures_iter(&line) {
            for name in re_block.capture_names() {
                match name {
                    Some(s) => match group.name(s) {
                        Some(gs) => {
                            let tokens: Option<Vec<token::Token>>;
                            let mtype: metadata::Metatype;
                            if s == "text" {
                                mtype = metadata::Metatype::TEXT;
                                tokens = None;
                            }
                            else if s == "code" {
                                let data = String::from(gs.as_str());
                                mtype = metadata::Metatype::CODE;
                                tokens = Some(tokenize(&data));
                            }
                            else if s == "echo" {
                                let data = String::from(gs.as_str());
                                mtype = metadata::Metatype::ECHO;
                                tokens = Some(tokenize(&data));
                            }
                            else {
                                mtype = metadata::Metatype::COMMENT;
                                tokens = None;
                            }

                            ret.push(metadata::Metadata::new(
                                mtype,
                                String::from(gs.as_str()),
                                tokens,
                            ));
                        }
                        None => continue,
                    }
                    None => continue,
                }
            }
        }

        ret
    }

    fn tokenize(code: &String) -> Vec<token::Token> {
        let mut ret: Vec<token::Token> = Vec::new();
        let re_internal = Regex::new(REG_INNER_BLOCK).unwrap();
        let mut iter = match re_internal.captures(code) {
            Some(cap) => match cap.name("code") {
                Some(name) => name.as_str().chars().peekable(),
                None => panic!("cannot retrieve code from block {}", code),
            }
            None => panic!("invalid code {}", code),
        };

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
                            ret.push(parse_string(&mut iter));
                            iter.next();
                        }
                        // digits
                        '0'..='9' => {
                            ret.push(parse_number(&mut iter));
                        }
                        // identifiers
                        'a'..='z' => {
                            ret.push(parse_id(&mut iter));
                        }
                        // operators and errors
                        _ => {
                            ret.push(parse_single_op(ch));
                            iter.next();
                        }
                    }
                }
            }
        }

        ret
    }

    fn parse_string<P>(iter: &mut P) -> token::Token
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

        token::Token::new(token_types::TokenTypes::STRING, Some(data))
    }

    fn parse_number<P>(iter: &mut P) -> token::Token
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

        token::Token::new(token_types::TokenTypes::NUMBER, Some(data.to_string()))
    }

    fn parse_id<P>(iter: &mut P) -> token::Token
    where P: PeekableIterator<Item=char> {
        let mut data: String = String::new();

        loop {
            match iter.peek() {
                Some(&ch) => {
                    match ch {
                        'a'..='z' | 'A'..='Z' | '_' => {
                            data.push(ch);
                        }
                        _ => break,
                    }
                },
                None => break,
            }
            iter.next();
        }

        match token_types::keyword_by_token(data.as_str()) {
            Some(tk) => {
                token::Token::new(tk, Some(data))
            },
            None => {
                token::Token::new(token_types::TokenTypes::IDENTIFIER, Some(data))
            }
        }
    }

    fn parse_single_op(op: char) -> token::Token {
        token::Token::new(match op {
            '+' => token_types::TokenTypes::PLUS,
            '-' => token_types::TokenTypes::MINUS,
            '/' => token_types::TokenTypes::SLASH,
            '%' => token_types::TokenTypes::PERCENT,
            '*' => token_types::TokenTypes::STAR,
            '=' => token_types::TokenTypes::ASSIGN,
            ',' => token_types::TokenTypes::COMMA,
            '(' => token_types::TokenTypes::LPAREN,
            '[' => token_types::TokenTypes::LBRACKET,
            ')' => token_types::TokenTypes::RPAREN,
            ']' => token_types::TokenTypes::RBRACKET,
            _ => panic!("invalid operator {}", op),
        }, Some(op.to_string()))
    }
}

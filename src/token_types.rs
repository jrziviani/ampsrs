use phf::phf_map;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenTypes {
    IDENTIFIER,
    VARIABLE,
    STRING,
    NUMBER,
    BOOLEAN,
    TRUE,
    FALSE,
    NIL,
    AND,
    OR,
    NOT,
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
    PLUS,
    MINUS,
    SLASH,
    STAR,
    PERCENT,
    ASSIGN,
    COMMA,
    IF,
    ELSE,
    ELIF,
    ENDIF,
    FOR,
    IN,
    ENDFOR,
    LPAREN,
    LBRACKET,
    RPAREN,
    RBRACKET,
    RANGE,
    PRINT,
    EXCEPT,
    INSERT,
    INVALID,
}

static KEYWORDS: phf::Map<&'static str, TokenTypes> = phf_map! {
    "boolean"   => TokenTypes::BOOLEAN,
    "true"      => TokenTypes::TRUE,
    "false"     => TokenTypes::FALSE,
    "null"      => TokenTypes::NIL,
    "and"       => TokenTypes::AND,
    "or"        => TokenTypes::OR,
    "not"       => TokenTypes::NOT,
    "eq"        => TokenTypes::EQ,
    "ne"        => TokenTypes::NE,
    "lt"        => TokenTypes::LT,
    "le"        => TokenTypes::LE,
    "gt"        => TokenTypes::GT,
    "ge"        => TokenTypes::GE,
    "if"        => TokenTypes::IF,
    "else"      => TokenTypes::ELSE,
    "elif"      => TokenTypes::ELIF,
    "endif"     => TokenTypes::ENDIF,
    "for"       => TokenTypes::FOR,
    "in"        => TokenTypes::IN,
    "endfor"    => TokenTypes::ENDFOR,
    "range"     => TokenTypes::RANGE,
    "print"     => TokenTypes::PRINT,
    "except"    => TokenTypes::EXCEPT,
    "insert"    => TokenTypes::INSERT,
};

pub fn keyword_by_token(token: &str) -> Option<TokenTypes> {
    KEYWORDS.get(token).cloned()
}

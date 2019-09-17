#[derive(Debug, PartialEq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    LeftCurlyBraces,
    RightCurlyBraces,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBrackets,
    RightSquareBrackets,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Ampersand,
    VerticalBar,
    LessThan,
    GreaterThan,
    Equal,
    Tilde,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword,
    Symbol,
    IntegerLiteral,
    StringLiteral,
    Identifier,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword, String),
    Symbol(Symbol, String),
    IntegerLiteral(u16),
    StringLiteral(String),
    Identifier(String),
}

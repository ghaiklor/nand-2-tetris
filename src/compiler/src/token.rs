#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword, String),
    Symbol(Symbol, String),
    IntegerLiteral(u16),
    StringLiteral(String),
    Identifier(String),
}

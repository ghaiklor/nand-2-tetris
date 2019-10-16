use crate::token::*;

pub struct Scanner {
    index: usize,
    source: String,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source_code: &str) -> Scanner {
        Scanner {
            index: 0,
            source: String::from(source_code),
            line: 1,
            column: 1,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let length = self.source.chars().count();

        while self.index < length {
            let character = self.current_char();
            let next_character = self.next_char();

            if character == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if character == '/' && next_character == '/' {
                self.skip_comments();
                continue;
            }

            if character == '/' && next_character == '*' {
                self.skip_comments();
                continue;
            }

            if character.is_whitespace() {
                self.index += 1;
                self.column += 1;
                continue;
            }

            if character.is_numeric() {
                tokens.push(self.scan_integer_literal());
                continue;
            }

            if character == '"' {
                self.index += 1;
                self.column += 1;
                tokens.push(self.scan_string_literal());
                continue;
            }

            if character.is_alphanumeric() {
                tokens.push(self.scan_identifier_or_keyword());
                continue;
            }

            tokens.push(self.scan_symbol());
        }

        tokens
    }

    fn current_char(&self) -> char {
        self.source
            .get(self.index..=self.index)
            .unwrap_or_else(|| "")
            .chars()
            .next()
            .unwrap_or_else(|| '\0')
    }

    fn next_char(&self) -> char {
        self.source
            .get(self.index + 1..=self.index + 1)
            .unwrap_or_else(|| "")
            .chars()
            .next()
            .unwrap_or_else(|| '\0')
    }

    fn skip_comments(&mut self) {
        let length = self.source.chars().count();
        let mut character = self.current_char();
        let mut next_character = self.next_char();

        if character == '/' && next_character == '/' {
            while character != '\n' && self.index < length {
                self.index += 1;
                self.column += 1;

                character = self.current_char();
            }

            self.index += 1;
            self.line += 1;
            self.column = 1;
        }

        if character == '/' && next_character == '*' {
            self.index += 2;
            self.column += 2;

            while !(character == '*' && next_character == '/') && self.index < length {
                self.index += 1;
                self.column += 1;
                if character == '\n' {
                    self.line += 1;
                    self.column = 1;
                }

                character = self.current_char();
                next_character = self.next_char();
            }

            self.index += 2;
            self.column += 2;
        }
    }

    fn scan_symbol(&mut self) -> Token {
        let character = self.current_char();
        self.index += 1;
        self.column += 1;

        match character {
            '{' => Token::Symbol(Symbol::LeftCurlyBraces, String::from("{")),
            '}' => Token::Symbol(Symbol::RightCurlyBraces, String::from("}")),
            '(' => Token::Symbol(Symbol::LeftParenthesis, String::from("(")),
            ')' => Token::Symbol(Symbol::RightParenthesis, String::from(")")),
            '[' => Token::Symbol(Symbol::LeftSquareBrackets, String::from("[")),
            ']' => Token::Symbol(Symbol::RightSquareBrackets, String::from("]")),
            '.' => Token::Symbol(Symbol::Dot, String::from(".")),
            ',' => Token::Symbol(Symbol::Comma, String::from(",")),
            ';' => Token::Symbol(Symbol::Semicolon, String::from(";")),
            '+' => Token::Symbol(Symbol::Plus, String::from("+")),
            '-' => Token::Symbol(Symbol::Minus, String::from("-")),
            '*' => Token::Symbol(Symbol::Asterisk, String::from("*")),
            '/' => Token::Symbol(Symbol::Slash, String::from("/")),
            '&' => Token::Symbol(Symbol::Ampersand, String::from("&")),
            '|' => Token::Symbol(Symbol::VerticalBar, String::from("|")),
            '<' => Token::Symbol(Symbol::LessThan, String::from("<")),
            '>' => Token::Symbol(Symbol::GreaterThan, String::from(">")),
            '=' => Token::Symbol(Symbol::Equal, String::from("=")),
            '~' => Token::Symbol(Symbol::Tilde, String::from("~")),
            _ => panic!(
                "Unknown character: {:?} at {}:{}",
                character, self.line, self.column
            ),
        }
    }

    fn scan_integer_literal(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self.current_char();

        while character.is_numeric() && self.index < length {
            buffer.push(character);
            self.index += 1;
            self.column += 1;
            character = self.current_char();
        }

        let integer: u16 = buffer.parse().expect("Integer literal expected");
        Token::IntegerLiteral(integer)
    }

    fn scan_string_literal(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self.current_char();

        while character != '"' && self.index < length {
            buffer.push(character);
            self.index += 1;
            self.column += 1;
            character = self.current_char();
        }

        self.index += 1;
        Token::StringLiteral(buffer)
    }

    fn scan_identifier_or_keyword(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self.current_char();

        while character.is_alphanumeric() && self.index < length {
            buffer.push(character);
            self.index += 1;
            self.column += 1;
            character = self.current_char();
        }

        match buffer.as_str() {
            "class" => Token::Keyword(Keyword::Class, buffer),
            "constructor" => Token::Keyword(Keyword::Constructor, buffer),
            "function" => Token::Keyword(Keyword::Function, buffer),
            "method" => Token::Keyword(Keyword::Method, buffer),
            "field" => Token::Keyword(Keyword::Field, buffer),
            "static" => Token::Keyword(Keyword::Static, buffer),
            "var" => Token::Keyword(Keyword::Var, buffer),
            "int" => Token::Keyword(Keyword::Int, buffer),
            "char" => Token::Keyword(Keyword::Char, buffer),
            "boolean" => Token::Keyword(Keyword::Boolean, buffer),
            "void" => Token::Keyword(Keyword::Void, buffer),
            "true" => Token::Keyword(Keyword::True, buffer),
            "false" => Token::Keyword(Keyword::False, buffer),
            "null" => Token::Keyword(Keyword::Null, buffer),
            "this" => Token::Keyword(Keyword::This, buffer),
            "let" => Token::Keyword(Keyword::Let, buffer),
            "do" => Token::Keyword(Keyword::Do, buffer),
            "if" => Token::Keyword(Keyword::If, buffer),
            "else" => Token::Keyword(Keyword::Else, buffer),
            "while" => Token::Keyword(Keyword::While, buffer),
            "return" => Token::Keyword(Keyword::Return, buffer),
            _ => Token::Identifier(buffer),
        }
    }
}
